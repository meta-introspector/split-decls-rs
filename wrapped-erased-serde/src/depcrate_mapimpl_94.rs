// Generated macro for impl_94 (impl)
macro_rules! Depcrate_mapimpl_94 {
() => {
// Module: crate::map
// Provides: {"impl_94"}
// Dependencies: {}
impl < T > OptionExt < T > for Option < T > { unsafe fn unsafe_map < U > (self , op : unsafe fn (T) -> U) -> Option < U > { match self { Some (t) => Some (unsafe { op (t) }) , None => None , } } }
};
}
