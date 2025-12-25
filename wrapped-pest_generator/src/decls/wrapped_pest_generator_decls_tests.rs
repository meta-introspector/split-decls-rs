use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(test)]
mod tests {
    /**Matches dar

    Match dar description
    */
    #[test]
    fn test_generate_doc() {
        let input = quote! {
            #[derive(Parser)] #[non_exhaustive] #[grammar = "../tests/test.pest"] pub
            struct TestParser;
        };
        let token = super::derive_parser(input, true);
        let expected = quote! {
            #[doc =
            "A parser for JSON file.\nAnd this is a example for JSON parser.\n\n    indent-4-space\n"]
            #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[non_exhaustive] pub enum Rule { #[doc = "Matches foo str, e.g.: `foo`"]
            r#foo, #[doc = "Matches bar str\n\n  Indent 2, e.g: `bar` or `foobar`"]
            r#bar, r#bar1, #[doc = "Matches dar\n\nMatch dar description\n"] r#dar }
        };
        assert!(
            token.to_string().contains(expected.to_string().as_str()),
            "{}\n\nExpected to contains:\n{}",
            token,
            expected
        );
    }
}
