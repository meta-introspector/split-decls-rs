// Generated macro for impl_196 (impl)
macro_rules! Depcrate_dictionaryimpl_196 {
() => {
// Module: crate::dictionary
// Provides: {"impl_196"}
// Dependencies: {}
# [cfg (feature = "NSEnumerator")] unsafe impl < KeyType : Message , ObjectType : Message > iter :: FastEnumerationHelper for NSMutableDictionary < KeyType , ObjectType > { type Item = KeyType ; # [inline] fn maybe_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
