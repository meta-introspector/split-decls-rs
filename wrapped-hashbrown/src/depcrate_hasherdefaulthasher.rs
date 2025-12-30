// Generated macro for DefaultHasher (struct)
macro_rules! Depcrate_hasherDefaultHasher {
() => {
// Module: crate::hasher
// Provides: {"DefaultHasher"}
// Dependencies: {}
# [doc = " Default hasher for [`HashMap`](crate::HashMap) and [`HashSet`](crate::HashSet)."] # [cfg (feature = "default-hasher")] # [derive (Clone)] pub struct DefaultHasher { inner : < RandomState as BuildHasher > :: Hasher , }
};
}
