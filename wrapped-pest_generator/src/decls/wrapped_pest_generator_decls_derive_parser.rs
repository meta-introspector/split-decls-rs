use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Processes the derive/proc macro input and generates the corresponding parser based
/// on the parsed grammar. If `include_grammar` is set to true, it'll generate an explicit
/// "include_str" statement (done in pest_derive, but turned off in the local bootstrap).
pub fn derive_parser(input: TokenStream, include_grammar: bool) -> TokenStream {
    let ast: DeriveInput = syn::parse2(input).unwrap();
    let (parsed_derive, contents) = parse_derive(ast);
    let mut data = String::new();
    let mut paths = vec![];
    for content in contents {
        let (_data, _path) = match content {
            GrammarSource::File(ref path) => {
                let root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
                let path = if Path::new(&root).join(path).exists() {
                    Path::new(&root).join(path)
                } else {
                    Path::new(&root).join("src/").join(path)
                };
                let file_name = match path.file_name() {
                    Some(file_name) => file_name,
                    None => panic!("grammar attribute should point to a file"),
                };
                let data = match read_file(&path) {
                    Ok(data) => data,
                    Err(error) => panic!("error opening {:?}: {}", file_name, error),
                };
                (data, Some(path.clone()))
            }
            GrammarSource::Inline(content) => (content, None),
        };
        data.push_str(&_data);
        if let Some(path) = _path {
            paths.push(path);
        }
    }
    let pairs = match parser::parse(Rule::grammar_rules, &data) {
        Ok(pairs) => pairs,
        Err(error) => panic!("error parsing \n{}", error.renamed_rules(rename_meta_rule)),
    };
    let defaults = unwrap_or_report(validator::validate_pairs(pairs.clone()));
    let doc_comment = docs::consume(pairs.clone());
    let ast = unwrap_or_report(parser::consume_rules(pairs));
    let optimized = optimizer::optimize(ast);
    generate(
        parsed_derive,
        paths,
        optimized,
        defaults,
        &doc_comment,
        include_grammar,
    )
}
