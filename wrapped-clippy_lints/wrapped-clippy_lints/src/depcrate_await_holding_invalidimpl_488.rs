// Generated macro for impl_488 (impl)
macro_rules! Depcrate_await_holding_invalidimpl_488 {
() => {
// Module: crate::await_holding_invalid
// Provides: {"impl_488"}
// Dependencies: {}
impl AwaitHolding { pub (crate) fn new (tcx : TyCtxt < '_ > , conf : & 'static Conf) -> Self { let (def_ids , _) = create_disallowed_map (tcx , & conf . await_holding_invalid_types , PathNS :: Type , crate :: disallowed_types :: def_kind_predicate , "type" , false ,) ; Self { def_ids } } }
};
}
