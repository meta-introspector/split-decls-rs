use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl AstDeclarationVisitor {
    fn new(current_file_path: PathBuf) -> Self {
        AstDeclarationVisitor {
            declarations: Vec::new(),
            current_file_path,
        }
    }
    fn add_declaration(
        &mut self,
        kind: String,
        name: String,
        item_attrs: &[syn::Attribute],
    ) {
        let is_public = item_attrs.iter().any(|attr| attr.path().is_ident("pub"));
        let mut attributes = HashSet::new();
        for attr in item_attrs {
            attributes.insert(attr.to_token_stream().to_string());
        }
        self.declarations
            .push(Declaration {
                kind,
                name,
                path: self.current_file_path.to_string_lossy().to_string(),
                semantic_hash: None,
                monster_factors: None,
                bag_of_words: None,
                eight_d_coordinate: None,
                deps: HashSet::new(),
                is_public,
                attributes,
            });
    }
}
