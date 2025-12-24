use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Macro {
    pub fn module(self, db: &dyn HirDatabase) -> Module {
        Module { id: self.id.module(db) }
    }
    pub fn name(self, db: &dyn HirDatabase) -> Name {
        match self.id {
            MacroId::Macro2Id(id) => {
                let loc = id.lookup(db);
                let source = loc.source(db);
                as_name_opt(source.value.name())
            }
            MacroId::MacroRulesId(id) => {
                let loc = id.lookup(db);
                let source = loc.source(db);
                as_name_opt(source.value.name())
            }
            MacroId::ProcMacroId(id) => {
                let loc = id.lookup(db);
                let source = loc.source(db);
                match loc.kind {
                    ProcMacroKind::CustomDerive => {
                        db.attrs(id.into())
                            .parse_proc_macro_derive()
                            .map_or_else(
                                || as_name_opt(source.value.name()),
                                |(it, _)| it,
                            )
                    }
                    ProcMacroKind::Bang | ProcMacroKind::Attr => {
                        as_name_opt(source.value.name())
                    }
                }
            }
        }
    }
    pub fn is_macro_export(self, db: &dyn HirDatabase) -> bool {
        matches!(
            self.id, MacroId::MacroRulesId(_) if db.attrs(self.id.into())
            .by_key(sym::macro_export).exists()
        )
    }
    pub fn is_proc_macro(self) -> bool {
        matches!(self.id, MacroId::ProcMacroId(_))
    }
    pub fn kind(&self, db: &dyn HirDatabase) -> MacroKind {
        match self.id {
            MacroId::Macro2Id(it) => {
                match it.lookup(db).expander {
                    MacroExpander::Declarative => MacroKind::Declarative,
                    MacroExpander::BuiltIn(_) | MacroExpander::BuiltInEager(_) => {
                        MacroKind::DeclarativeBuiltIn
                    }
                    MacroExpander::BuiltInAttr(_) => MacroKind::AttrBuiltIn,
                    MacroExpander::BuiltInDerive(_) => MacroKind::DeriveBuiltIn,
                }
            }
            MacroId::MacroRulesId(it) => {
                match it.lookup(db).expander {
                    MacroExpander::Declarative => MacroKind::Declarative,
                    MacroExpander::BuiltIn(_) | MacroExpander::BuiltInEager(_) => {
                        MacroKind::DeclarativeBuiltIn
                    }
                    MacroExpander::BuiltInAttr(_) => MacroKind::AttrBuiltIn,
                    MacroExpander::BuiltInDerive(_) => MacroKind::DeriveBuiltIn,
                }
            }
            MacroId::ProcMacroId(it) => {
                match it.lookup(db).kind {
                    ProcMacroKind::CustomDerive => MacroKind::Derive,
                    ProcMacroKind::Bang => MacroKind::ProcMacro,
                    ProcMacroKind::Attr => MacroKind::Attr,
                }
            }
        }
    }
    pub fn is_fn_like(&self, db: &dyn HirDatabase) -> bool {
        matches!(
            self.kind(db), MacroKind::Declarative | MacroKind::DeclarativeBuiltIn |
            MacroKind::ProcMacro
        )
    }
    pub fn is_builtin_derive(&self, db: &dyn HirDatabase) -> bool {
        match self.id {
            MacroId::Macro2Id(it) => {
                matches!(it.lookup(db).expander, MacroExpander::BuiltInDerive(_))
            }
            MacroId::MacroRulesId(it) => {
                matches!(it.lookup(db).expander, MacroExpander::BuiltInDerive(_))
            }
            MacroId::ProcMacroId(_) => false,
        }
    }
    pub fn is_env_or_option_env(&self, db: &dyn HirDatabase) -> bool {
        match self.id {
            MacroId::Macro2Id(it) => {
                matches!(
                    it.lookup(db).expander, MacroExpander::BuiltInEager(eager) if eager
                    .is_env_or_option_env()
                )
            }
            MacroId::MacroRulesId(it) => {
                matches!(
                    it.lookup(db).expander, MacroExpander::BuiltInEager(eager) if eager
                    .is_env_or_option_env()
                )
            }
            MacroId::ProcMacroId(_) => false,
        }
    }
    /// Is this `asm!()`, or a variant of it (e.g. `global_asm!()`)?
    pub fn is_asm_like(&self, db: &dyn HirDatabase) -> bool {
        match self.id {
            MacroId::Macro2Id(it) => {
                matches!(it.lookup(db).expander, MacroExpander::BuiltIn(m) if m.is_asm())
            }
            MacroId::MacroRulesId(it) => {
                matches!(it.lookup(db).expander, MacroExpander::BuiltIn(m) if m.is_asm())
            }
            MacroId::ProcMacroId(_) => false,
        }
    }
    pub fn is_attr(&self, db: &dyn HirDatabase) -> bool {
        matches!(self.kind(db), MacroKind::Attr | MacroKind::AttrBuiltIn)
    }
    pub fn is_derive(&self, db: &dyn HirDatabase) -> bool {
        matches!(self.kind(db), MacroKind::Derive | MacroKind::DeriveBuiltIn)
    }
}
