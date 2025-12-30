// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl uWrite for W { type Error = Infallible ; fn write_str (& mut self , s : & str) -> Result < () , Infallible > { s . as_bytes () . iter () . for_each (| b | unsafe { let _ = ptr :: read_volatile (b) ; }) ; Ok (()) } }
};
}
