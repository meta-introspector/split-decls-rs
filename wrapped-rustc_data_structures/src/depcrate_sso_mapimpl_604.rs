// Generated macro for impl_604 (impl)
macro_rules! Depcrate_sso_mapimpl_604 {
() => {
// Module: crate::sso::map
// Provides: {"impl_604"}
// Dependencies: {}
impl < 'a , K : Eq + Hash , V : Default > Entry < 'a , K , V > { # [doc = " Ensures a value is in the entry by inserting the default value if empty,"] # [doc = " and returns a mutable reference to the value in the entry."] # [inline] pub fn or_default (self) -> & 'a mut V { self . or_insert_with (Default :: default) } }
};
}
