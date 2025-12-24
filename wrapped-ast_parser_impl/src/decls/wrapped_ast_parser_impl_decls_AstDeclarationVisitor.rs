use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A `syn::visit::Visit` implementation to extract `Declaration`s from a `syn::File`.
#[derive(Debug, Default)]
struct AstDeclarationVisitor {
    declarations: Vec<Declaration>,
    current_file_path: PathBuf,
}
