// Generated macro for impl_71 (impl)
macro_rules! Depcrate_typesimpl_71 {
() => {
// Module: crate::types
// Provides: {"impl_71"}
// Dependencies: {}
impl < 'a > Glob < 'a > { fn unmatched () -> Glob < 'a > { Glob (GlobInner :: UnmatchedIgnore) } # [doc = " Return the file type definition that matched, if one exists. A file type"] # [doc = " definition always exists when a specific definition matches a file"] # [doc = " path."] pub fn file_type_def (& self) -> Option < & FileTypeDef > { match self { Glob (GlobInner :: UnmatchedIgnore) => None , Glob (GlobInner :: Matched { def , .. }) => Some (def) , } } }
};
}
