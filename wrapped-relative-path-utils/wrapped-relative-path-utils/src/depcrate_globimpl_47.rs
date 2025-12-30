// Generated macro for impl_47 (impl)
macro_rules! Depcrate_globimpl_47 {
() => {
// Module: crate::glob
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a > Glob < 'a > { # [doc = " Construct a new glob pattern."] pub (super) fn new (root : & 'a Root , pattern : & 'a RelativePath) -> Self { let components = compile_pattern (pattern) ; Self { root , components } } # [doc = " Construct a new matcher over the compiled glob pattern."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use relative_path_utils::Root;"] # [doc = ""] # [doc = " let root = Root::new(\"src\")?;"] # [doc = ""] # [doc = " let glob = root.glob(\"**/*.rs\");"] # [doc = ""] # [doc = " let mut results = Vec::new();"] # [doc = ""] # [doc = " for e in glob.matcher() {"] # [doc = "     results.push(e?);"] # [doc = " }"] # [doc = ""] # [doc = " results.sort();"] # [doc = " assert_eq!(results, vec![\"lib.rs\", \"main.rs\"]);"] # [doc = " # Ok::<_, Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [must_use] pub fn matcher (& self) -> Matcher < '_ > { Matcher { root : self . root , queue : [(RelativePathBuf :: new () , self . components . as_ref ())] . into_iter () . collect () , } } }
};
}
