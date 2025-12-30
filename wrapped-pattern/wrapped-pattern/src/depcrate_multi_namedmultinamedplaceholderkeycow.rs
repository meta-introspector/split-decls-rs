// Generated macro for MultiNamedPlaceholderKeyCow (struct)
macro_rules! Depcrate_multi_namedMultiNamedPlaceholderKeyCow {
() => {
// Module: crate::multi_named
// Provides: {"MultiNamedPlaceholderKeyCow"}
// Dependencies: {}
# [doc = " Cowable version of [`MultiNamedPlaceholderKey`], used during construction."] # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize , serde :: Serialize))] # [repr (transparent)] # [allow (clippy :: exhaustive_structs)] # [cfg (feature = "alloc")] pub struct MultiNamedPlaceholderKeyCow < 'a > (pub Cow < 'a , str >) ;
};
}
