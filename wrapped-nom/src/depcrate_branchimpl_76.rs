// Generated macro for impl_76 (impl)
macro_rules! Depcrate_branchimpl_76 {
() => {
// Module: crate::branch
// Provides: {"impl_76"}
// Dependencies: {}
impl < Input , Output , Error : ParseError < Input > , A : Parser < Input , Output = Output , Error = Error > > Parser < Input > for Choice < (A ,) > { type Output = Output ; type Error = Error ; # [inline] fn process < OM : crate :: OutputMode > (& mut self , input : Input ,) -> crate :: PResult < OM , Input , Self :: Output , Self :: Error > { self . parser . 0 . process :: < OM > (input) } }
};
}
