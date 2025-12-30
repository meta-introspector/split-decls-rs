// Generated macro for impl_416 (impl)
macro_rules! Depcrate_commitimpl_416 {
() => {
// Module: crate::commit
// Provides: {"impl_416"}
// Dependencies: {}
impl < 'repo > std :: fmt :: Debug for Commit < 'repo > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { let mut ds = f . debug_struct ("Commit") ; ds . field ("id" , & self . id ()) ; if let Some (summary) = self . summary () { ds . field ("summary" , & summary) ; } ds . finish () } }
};
}
