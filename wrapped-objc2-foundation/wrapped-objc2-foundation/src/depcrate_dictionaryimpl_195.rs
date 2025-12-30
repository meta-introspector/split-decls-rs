// Generated macro for impl_195 (impl)
macro_rules! Depcrate_dictionaryimpl_195 {
() => {
// Module: crate::dictionary
// Provides: {"impl_195"}
// Dependencies: {}
# [cfg (feature = "NSEnumerator")] unsafe impl < KeyType : Message , ObjectType : Message > iter :: FastEnumerationHelper for NSDictionary < KeyType , ObjectType > { type Item = KeyType ; # [inline] fn maybe_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
