// Generated macro for impl_987 (impl)
macro_rules! Depcrate_machineimpl_987 {
() => {
// Module: crate::machine
// Provides: {"impl_987"}
// Dependencies: {}
impl fmt :: Debug for Provenance { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Provenance :: Concrete { alloc_id , tag } => { if f . alternate () { write ! (f , "[{alloc_id:#?}]") ? ; } else { write ! (f , "[{alloc_id:?}]") ? ; } write ! (f , "{tag:?}") ? ; } Provenance :: Wildcard => { write ! (f , "[wildcard]") ? ; } } Ok (()) } }
};
}
