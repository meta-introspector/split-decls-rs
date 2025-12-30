// Generated macro for impl_117 (impl)
macro_rules! Depcrate_defimpl_117 {
() => {
// Module: crate::def
// Provides: {"impl_117"}
// Dependencies: {}
impl < T > :: std :: ops :: IndexMut < Namespace > for PerNS < T > { fn index_mut (& mut self , ns : Namespace) -> & mut T { match ns { Namespace :: ValueNS => & mut self . value_ns , Namespace :: TypeNS => & mut self . type_ns , Namespace :: MacroNS => & mut self . macro_ns , } } }
};
}
