// Generated macro for impl_80 (impl)
macro_rules! Depcrateimpl_80 {
() => {
// Module: crate
// Provides: {"impl_80"}
// Dependencies: {}
impl GlobSetBuilder { # [doc = " Create a new `GlobSetBuilder`. A `GlobSetBuilder` can be used to add new"] # [doc = " patterns. Once all patterns have been added, `build` should be called"] # [doc = " to produce a [`GlobSet`], which can then be used for matching."] pub fn new () -> GlobSetBuilder { GlobSetBuilder { pats : vec ! [] } } # [doc = " Builds a new matcher from all of the glob patterns added so far."] # [doc = ""] # [doc = " Once a matcher is built, no new patterns can be added to it."] pub fn build (& self) -> Result < GlobSet , Error > { GlobSet :: new (self . pats . iter ()) } # [doc = " Add a new pattern to this set."] pub fn add (& mut self , pat : Glob) -> & mut GlobSetBuilder { self . pats . push (pat) ; self } }
};
}
