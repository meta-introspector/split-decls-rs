use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl MacroDefId {
    pub fn make_call(
        self,
        db: &dyn ExpandDatabase,
        krate: Crate,
        kind: MacroCallKind,
        ctxt: SyntaxContext,
    ) -> MacroCallId {
        db.intern_macro_call(MacroCallLoc {
            def: self,
            krate,
            kind,
            ctxt,
        })
    }
    pub fn definition_range(&self, db: &dyn ExpandDatabase) -> InFile<TextRange> {
        match self.kind {
            MacroDefKind::Declarative(id)
            | MacroDefKind::BuiltIn(id, _)
            | MacroDefKind::BuiltInAttr(id, _)
            | MacroDefKind::BuiltInDerive(id, _)
            | MacroDefKind::BuiltInEager(id, _) => {
                id.with_value(db.ast_id_map(id.file_id).get(id.value).text_range())
            }
            MacroDefKind::ProcMacro(id, _, _) => {
                id.with_value(db.ast_id_map(id.file_id).get(id.value).text_range())
            }
        }
    }
    pub fn ast_id(&self) -> Either<AstId<ast::Macro>, AstId<ast::Fn>> {
        match self.kind {
            MacroDefKind::ProcMacro(id, ..) => Either::Right(id),
            MacroDefKind::Declarative(id)
            | MacroDefKind::BuiltIn(id, _)
            | MacroDefKind::BuiltInAttr(id, _)
            | MacroDefKind::BuiltInDerive(id, _)
            | MacroDefKind::BuiltInEager(id, _) => Either::Left(id),
        }
    }
    pub fn is_proc_macro(&self) -> bool {
        matches!(self.kind, MacroDefKind::ProcMacro(..))
    }
    pub fn is_attribute(&self) -> bool {
        matches!(
            self.kind,
            MacroDefKind::BuiltInAttr(..) | MacroDefKind::ProcMacro(_, _, ProcMacroKind::Attr)
        )
    }
    pub fn is_derive(&self) -> bool {
        matches!(
            self.kind,
            MacroDefKind::BuiltInDerive(..)
                | MacroDefKind::ProcMacro(_, _, ProcMacroKind::CustomDerive)
        )
    }
    pub fn is_fn_like(&self) -> bool {
        matches!(
            self.kind,
            MacroDefKind::BuiltIn(..)
                | MacroDefKind::ProcMacro(_, _, ProcMacroKind::Bang)
                | MacroDefKind::BuiltInEager(..)
                | MacroDefKind::Declarative(..)
        )
    }
    pub fn is_attribute_derive(&self) -> bool {
        matches!(
            self.kind, MacroDefKind::BuiltInAttr(_, expander) if expander.is_derive()
        )
    }
    pub fn is_include(&self) -> bool {
        matches!(
            self.kind, MacroDefKind::BuiltInEager(_, expander) if expander.is_include()
        )
    }
    pub fn is_include_like(&self) -> bool {
        matches!(
            self.kind, MacroDefKind::BuiltInEager(_, expander) if expander
            .is_include_like()
        )
    }
    pub fn is_env_or_option_env(&self) -> bool {
        matches!(
            self.kind, MacroDefKind::BuiltInEager(_, expander) if expander
            .is_env_or_option_env()
        )
    }
}
