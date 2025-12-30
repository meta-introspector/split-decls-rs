// Generated macro for impl_116 (impl)
macro_rules! Depcrate_defimpl_116 {
() => {
// Module: crate::def
// Provides: {"impl_116"}
// Dependencies: {}
impl < T > :: std :: ops :: Index < Namespace > for PerNS < T > { type Output = T ; fn index (& self , ns : Namespace) -> & T { match ns { Namespace :: ValueNS => & self . value_ns , Namespace :: TypeNS => & self . type_ns , Namespace :: MacroNS => & self . macro_ns , } } }
};
}
