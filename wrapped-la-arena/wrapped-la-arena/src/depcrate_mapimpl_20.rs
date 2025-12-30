// Generated macro for impl_20 (impl)
macro_rules! Depcrate_mapimpl_20 {
() => {
// Module: crate::map
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'a , IDX , V > Entry < 'a , IDX , V > where V : Default , { # [doc = " Ensures a value is in the entry by inserting the default value if empty, and returns a mutable reference"] # [doc = " to the value in the entry."] # [allow (clippy :: unwrap_or_default)] pub fn or_default (self) -> & 'a mut V { self . or_insert_with (Default :: default) } }
};
}
