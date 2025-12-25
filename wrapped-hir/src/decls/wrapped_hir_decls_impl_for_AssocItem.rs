use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl AssocItem {
    pub fn name(self, db: &dyn HirDatabase) -> Option<Name> {
        match self {
            AssocItem::Function(it) => Some(it.name(db)),
            AssocItem::Const(it) => it.name(db),
            AssocItem::TypeAlias(it) => Some(it.name(db)),
        }
    }
    pub fn module(self, db: &dyn HirDatabase) -> Module {
        match self {
            AssocItem::Function(f) => f.module(db),
            AssocItem::Const(c) => c.module(db),
            AssocItem::TypeAlias(t) => t.module(db),
        }
    }
    pub fn container(self, db: &dyn HirDatabase) -> AssocItemContainer {
        let container = match self {
            AssocItem::Function(it) => it.id.lookup(db).container,
            AssocItem::Const(it) => it.id.lookup(db).container,
            AssocItem::TypeAlias(it) => it.id.lookup(db).container,
        };
        match container {
            ItemContainerId::TraitId(id) => AssocItemContainer::Trait(id.into()),
            ItemContainerId::ImplId(id) => AssocItemContainer::Impl(id.into()),
            ItemContainerId::ModuleId(_) | ItemContainerId::ExternBlockId(_) => {
                panic!("invalid AssocItem")
            }
        }
    }
    pub fn container_trait(self, db: &dyn HirDatabase) -> Option<Trait> {
        match self.container(db) {
            AssocItemContainer::Trait(t) => Some(t),
            _ => None,
        }
    }
    pub fn implemented_trait(self, db: &dyn HirDatabase) -> Option<Trait> {
        match self.container(db) {
            AssocItemContainer::Impl(i) => i.trait_(db),
            _ => None,
        }
    }
    pub fn container_or_implemented_trait(self, db: &dyn HirDatabase) -> Option<Trait> {
        match self.container(db) {
            AssocItemContainer::Trait(t) => Some(t),
            AssocItemContainer::Impl(i) => i.trait_(db),
        }
    }
    pub fn implementing_ty(self, db: &dyn HirDatabase) -> Option<Type<'_>> {
        match self.container(db) {
            AssocItemContainer::Impl(i) => Some(i.self_ty(db)),
            _ => None,
        }
    }
    pub fn as_function(self) -> Option<Function> {
        match self {
            Self::Function(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_const(self) -> Option<Const> {
        match self {
            Self::Const(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_type_alias(self) -> Option<TypeAlias> {
        match self {
            Self::TypeAlias(v) => Some(v),
            _ => None,
        }
    }
    pub fn diagnostics<'db>(
        self,
        db: &'db dyn HirDatabase,
        acc: &mut Vec<AnyDiagnostic<'db>>,
        style_lints: bool,
    ) {
        match self {
            AssocItem::Function(func) => {
                GenericDef::Function(func).diagnostics(db, acc);
                DefWithBody::from(func).diagnostics(db, acc, style_lints);
            }
            AssocItem::Const(const_) => {
                GenericDef::Const(const_).diagnostics(db, acc);
                DefWithBody::from(const_).diagnostics(db, acc, style_lints);
            }
            AssocItem::TypeAlias(type_alias) => {
                GenericDef::TypeAlias(type_alias).diagnostics(db, acc);
                push_ty_diagnostics(
                    db,
                    acc,
                    db.type_for_type_alias_with_diagnostics(type_alias.id).1,
                    &db.type_alias_signature_with_source_map(type_alias.id).1,
                );
                for diag in hir_ty::diagnostics::incorrect_case(db, type_alias.id.into()) {
                    acc.push(diag.into());
                }
            }
        }
    }
}
