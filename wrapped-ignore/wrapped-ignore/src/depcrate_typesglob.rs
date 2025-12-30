// Generated macro for Glob (struct)
macro_rules! Depcrate_typesGlob {
() => {
// Module: crate::types
// Provides: {"Glob"}
// Dependencies: {}
# [doc = " Glob represents a single glob in a set of file type definitions."] # [doc = ""] # [doc = " There may be more than one glob for a particular file type."] # [doc = ""] # [doc = " This is used to report information about the highest precedent glob"] # [doc = " that matched."] # [doc = ""] # [doc = " Note that not all matches necessarily correspond to a specific glob."] # [doc = " For example, if there are one or more selections and a file path doesn't"] # [doc = " match any of those selections, then the file path is considered to be"] # [doc = " ignored."] # [doc = ""] # [doc = " The lifetime `'a` refers to the lifetime of the underlying file type"] # [doc = " definition, which corresponds to the lifetime of the file type matcher."] # [derive (Clone , Debug)] pub struct Glob < 'a > (GlobInner < 'a >) ;
};
}
