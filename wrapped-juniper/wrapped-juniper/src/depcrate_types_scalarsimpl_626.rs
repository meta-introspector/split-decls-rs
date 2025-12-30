// Generated macro for impl_626 (impl)
macro_rules! Depcrate_types_scalarsimpl_626 {
() => {
// Module: crate::types::scalars
// Provides: {"impl_626"}
// Dependencies: {}
impl ID { # [doc = " Construct a new [`ID`] from anything implementing [`Into`]`<`[`String`]`>`."] # [must_use] pub fn new < S : Into < String > > (value : S) -> Self { ID (value . into () . into ()) } }
};
}
