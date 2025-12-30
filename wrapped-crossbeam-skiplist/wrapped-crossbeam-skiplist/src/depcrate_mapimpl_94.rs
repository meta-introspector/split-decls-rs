// Generated macro for impl_94 (impl)
macro_rules! Depcrate_mapimpl_94 {
() => {
// Module: crate::map
// Provides: {"impl_94"}
// Dependencies: {}
impl < K , V > Entry < '_ , K , V > where K : Ord + Send + 'static , V : Send + 'static , { # [doc = " Removes the entry from the map."] # [doc = ""] # [doc = " Returns `true` if this call removed the entry and `false` if it was already removed."] pub fn remove (& self) -> bool { let guard = & epoch :: pin () ; self . inner . remove (guard) } }
};
}
