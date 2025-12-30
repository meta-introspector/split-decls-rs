// Generated macro for impl_177 (impl)
macro_rules! Depcrate_setimpl_177 {
() => {
// Module: crate::set
// Provides: {"impl_177"}
// Dependencies: {}
impl < K : Eq + Hash , S : BuildHasher + Clone > PartialEq for DashSet < K , S > { fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . iter () . all (| r | other . contains (r . key ())) } }
};
}
