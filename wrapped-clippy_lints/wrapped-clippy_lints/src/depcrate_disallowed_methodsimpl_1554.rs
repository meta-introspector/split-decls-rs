// Generated macro for impl_1554 (impl)
macro_rules! Depcrate_disallowed_methodsimpl_1554 {
() => {
// Module: crate::disallowed_methods
// Provides: {"impl_1554"}
// Dependencies: {}
impl DisallowedMethods { pub fn new (tcx : TyCtxt < '_ > , conf : & 'static Conf) -> Self { let (disallowed , _) = create_disallowed_map (tcx , & conf . disallowed_methods , PathNS :: Value , | def_kind | { matches ! (def_kind , DefKind :: Fn | DefKind :: Ctor (_ , CtorKind :: Fn) | DefKind :: AssocFn) } , "function" , false ,) ; Self { disallowed } } }
};
}
