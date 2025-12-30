// Generated macro for impl_846 (impl)
macro_rules! Depcrate_core_builder_cargoimpl_846 {
() => {
// Module: crate::core::builder::cargo
// Provides: {"impl_846"}
// Dependencies: {}
impl HostFlags { const SEPARATOR : & 'static str = " " ; # [doc = " Adds a host rustc flag."] fn arg < S : Into < String > > (& mut self , flag : S) { let value = flag . into () . trim () . to_string () ; assert ! (! value . contains (Self :: SEPARATOR)) ; self . rustc . push (value) ; } # [doc = " Encodes all the flags into a single string."] fn encode (self) -> String { self . rustc . join (Self :: SEPARATOR) } }
};
}
