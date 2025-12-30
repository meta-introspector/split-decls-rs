// Generated macro for impl_36 (impl)
macro_rules! Depcrate_navigation_targetimpl_36 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_36"}
// Dependencies: {}
impl TryToNav for Definition { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { match self { Definition :: Local (it) => Some (it . to_nav (sema . db)) , Definition :: Label (it) => it . try_to_nav (sema) , Definition :: Module (it) => Some (it . to_nav (sema . db)) , Definition :: Crate (it) => Some (it . to_nav (sema . db)) , Definition :: Macro (it) => it . try_to_nav (sema) , Definition :: Field (it) => it . try_to_nav (sema) , Definition :: SelfType (it) => it . try_to_nav (sema) , Definition :: GenericParam (it) => it . try_to_nav (sema) , Definition :: Function (it) => it . try_to_nav (sema) , Definition :: Adt (it) => it . try_to_nav (sema) , Definition :: Variant (it) => it . try_to_nav (sema) , Definition :: Const (it) => it . try_to_nav (sema) , Definition :: Static (it) => it . try_to_nav (sema) , Definition :: Trait (it) => it . try_to_nav (sema) , Definition :: TypeAlias (it) => it . try_to_nav (sema) , Definition :: ExternCrateDecl (it) => it . try_to_nav (sema) , Definition :: InlineAsmOperand (it) => it . try_to_nav (sema) , Definition :: BuiltinType (it) => it . try_to_nav (sema) , Definition :: BuiltinLifetime (_) | Definition :: TupleField (_) | Definition :: ToolModule (_) | Definition :: InlineAsmRegOrRegClass (_) | Definition :: BuiltinAttr (_) => None , Definition :: DeriveHelper (it) => it . derive () . try_to_nav (sema) , } } }
};
}
