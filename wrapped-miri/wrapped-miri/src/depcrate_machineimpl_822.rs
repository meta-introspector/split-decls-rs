// Generated macro for impl_822 (impl)
macro_rules! Depcrate_machineimpl_822 {
() => {
// Module: crate::machine
// Provides: {"impl_822"}
// Dependencies: {}
impl fmt :: Debug for Provenance { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Provenance :: Concrete { alloc_id , tag } => { if f . alternate () { write ! (f , "[{alloc_id:#?}]") ? ; } else { write ! (f , "[{alloc_id:?}]") ? ; } write ! (f , "{tag:?}") ? ; } Provenance :: Wildcard => { write ! (f , "[wildcard]") ? ; } } Ok (()) } }
};
}
