// Generated macro for Map (struct)
macro_rules! Depcrate_value_mapMap {
() => {
// Module: crate::value::map
// Provides: {"Map"}
// Dependencies: {}
# [doc = " A [`Value`] to [`Value`] map."] # [doc = ""] # [doc = " This structure either uses a [`BTreeMap`](alloc::collections::BTreeMap) or the"] # [doc = " [`IndexMap`](indexmap::IndexMap) internally."] # [doc = " The latter can be used by enabling the `indexmap` feature. This can be used"] # [doc = " to preserve the order of the parsed map."] # [derive (Clone , Debug , Default , Deserialize , Serialize)] # [serde (transparent)] pub struct Map (pub (crate) MapInner) ;
};
}
