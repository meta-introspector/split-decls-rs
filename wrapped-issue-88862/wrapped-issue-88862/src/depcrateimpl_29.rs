// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl < SF , Req > ServiceFactory < Req > for FactoryWrapper < SF > where SF : ServiceFactory < Req > , SF : 'static , { type Future = BoxFuture < Result < Self :: Service , () > > ; type Service = Box < dyn Service < Req , Error = () , Future = BoxFuture < Result < () , () > > > > ; fn new_service (& self , _ : ()) -> Self :: Future { todo ! () } }
};
}
