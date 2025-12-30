// Generated macro for impl_845 (impl)
macro_rules! Depcrate_strategy_fuseimpl_845 {
() => {
// Module: crate::strategy::fuse
// Provides: {"impl_845"}
// Dependencies: {}
impl < T : ValueTree > Fuse < T > { # [doc = " Return whether a call to `simplify()` may be productive."] # [doc = ""] # [doc = " Formally, this is true if one of the following holds:"] # [doc = ""] # [doc = " - `simplify()` has never been called."] # [doc = " - The most recent call to `simplify()` returned `true`."] # [doc = " - `complicate()` has been called more recently than `simplify()` and"] # [doc = "   the last call returned `true`."] pub fn may_simplify (& self) -> bool { self . may_simplify } # [doc = " Disallow any further calls to `simplify()` until a call to"] # [doc = " `complicate()` returns `true`."] pub fn disallow_simplify (& mut self) { self . may_simplify = false ; } # [doc = " Return whether a call to `complicate()` may be productive."] # [doc = ""] # [doc = " Formally, this is true if one of the following holds:"] # [doc = ""] # [doc = " - The most recent call to `complicate()` returned `true`."] # [doc = " - `simplify()` has been called more recently than `complicate()` and"] # [doc = "   the last call returned `true`."] pub fn may_complicate (& self) -> bool { self . may_complicate } # [doc = " Disallow any further calls to `complicate()` until a call to"] # [doc = " `simplify()` returns `true`."] pub fn disallow_complicate (& mut self) { self . may_complicate = false ; } # [doc = " Prevent any further shrinking operations from occurring."] pub fn freeze (& mut self) { self . disallow_simplify () ; self . disallow_complicate () ; } }
};
}
