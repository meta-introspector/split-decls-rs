// Generated macro for BasePath (struct)
macro_rules! Depcrate_baseBasePath {
() => {
// Module: crate::base
// Provides: {"BasePath"}
// Dependencies: {}
# [doc = " A borrowed path that has a [prefix] on Windows."] # [doc = ""] # [doc = " Note that comparison traits such as [`PartialEq`] will compare paths"] # [doc = " literally instead of comparing components. The former is more efficient and"] # [doc = " easier to use correctly."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This type should not be used for memory safety, but implementations can"] # [doc = " panic if this path is missing a prefix on Windows. A safe `new_unchecked`"] # [doc = " method might be added later that can safely create invalid base paths."] # [doc = ""] # [doc = " [prefix]: ::std::path::Prefix"] # [derive (Debug , Eq , Hash , Ord , PartialEq , PartialOrd)] # [repr (transparent)] pub struct BasePath (pub (super) OsStr) ;
};
}
