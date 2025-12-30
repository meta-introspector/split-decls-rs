// Generated macro for impl_670 (impl)
macro_rules! Depcrate_interpret_stackimpl_670 {
() => {
// Module: crate::interpret::stack
// Provides: {"impl_670"}
// Dependencies: {}
impl < Prov : Provenance > std :: fmt :: Debug for LocalState < '_ , Prov > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("LocalState") . field ("value" , & self . value) . field ("ty" , & self . layout . get () . map (| l | l . ty)) . finish () } }
};
}
