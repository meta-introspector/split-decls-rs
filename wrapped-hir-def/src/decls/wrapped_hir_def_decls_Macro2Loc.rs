use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Macro2Loc {
    pub container: ModuleId,
    pub id: AstId<ast::MacroDef>,
    pub expander: MacroExpander,
    pub allow_internal_unsafe: bool,
    pub edition: Edition,
}
