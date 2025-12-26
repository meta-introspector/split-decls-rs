use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl MacroCallId {
    pub fn call_node(self, db: &dyn ExpandDatabase) -> InFile<SyntaxNode> {
        db.lookup_intern_macro_call(self).to_node(db)
    }
    pub fn expansion_level(self, db: &dyn ExpandDatabase) -> u32 {
        let mut level = 0;
        let mut macro_file = self;
        loop {
            let loc = db.lookup_intern_macro_call(macro_file);
            level += 1;
            macro_file = match loc.kind.file_id() {
                HirFileId::FileId(_) => break level,
                HirFileId::MacroFile(it) => it,
            };
        }
    }
    pub fn parent(self, db: &dyn ExpandDatabase) -> HirFileId {
        db.lookup_intern_macro_call(self).kind.file_id()
    }
    /// Return expansion information if it is a macro-expansion file
    pub fn expansion_info(self, db: &dyn ExpandDatabase) -> ExpansionInfo {
        ExpansionInfo::new(db, self)
    }
    pub fn kind(self, db: &dyn ExpandDatabase) -> MacroKind {
        match db.lookup_intern_macro_call(self).def.kind {
            MacroDefKind::Declarative(..) => MacroKind::Declarative,
            MacroDefKind::BuiltIn(..) | MacroDefKind::BuiltInEager(..) => {
                MacroKind::DeclarativeBuiltIn
            }
            MacroDefKind::BuiltInDerive(..) => MacroKind::DeriveBuiltIn,
            MacroDefKind::ProcMacro(_, _, ProcMacroKind::CustomDerive) => MacroKind::Derive,
            MacroDefKind::ProcMacro(_, _, ProcMacroKind::Attr) => MacroKind::Attr,
            MacroDefKind::ProcMacro(_, _, ProcMacroKind::Bang) => MacroKind::ProcMacro,
            MacroDefKind::BuiltInAttr(..) => MacroKind::AttrBuiltIn,
        }
    }
    pub fn is_include_macro(self, db: &dyn ExpandDatabase) -> bool {
        db.lookup_intern_macro_call(self).def.is_include()
    }
    pub fn is_include_like_macro(self, db: &dyn ExpandDatabase) -> bool {
        db.lookup_intern_macro_call(self).def.is_include_like()
    }
    pub fn is_env_or_option_env(self, db: &dyn ExpandDatabase) -> bool {
        db.lookup_intern_macro_call(self).def.is_env_or_option_env()
    }
    pub fn is_eager(self, db: &dyn ExpandDatabase) -> bool {
        let loc = db.lookup_intern_macro_call(self);
        matches!(loc.def.kind, MacroDefKind::BuiltInEager(..))
    }
    pub fn eager_arg(self, db: &dyn ExpandDatabase) -> Option<MacroCallId> {
        let loc = db.lookup_intern_macro_call(self);
        match &loc.kind {
            MacroCallKind::FnLike { eager, .. } => eager.as_ref().map(|it| it.arg_id),
            _ => None,
        }
    }
    pub fn is_derive_attr_pseudo_expansion(self, db: &dyn ExpandDatabase) -> bool {
        let loc = db.lookup_intern_macro_call(self);
        loc.def.is_attribute_derive()
    }
}
