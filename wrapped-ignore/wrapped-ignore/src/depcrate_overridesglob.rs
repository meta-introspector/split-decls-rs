// Generated macro for Glob (struct)
macro_rules! Depcrate_overridesGlob {
() => {
// Module: crate::overrides
// Provides: {"Glob"}
// Dependencies: {}
# [doc = " Glob represents a single glob in an override matcher."] # [doc = ""] # [doc = " This is used to report information about the highest precedent glob"] # [doc = " that matched."] # [doc = ""] # [doc = " Note that not all matches necessarily correspond to a specific glob. For"] # [doc = " example, if there are one or more whitelist globs and a file path doesn't"] # [doc = " match any glob in the set, then the file path is considered to be ignored."] # [doc = ""] # [doc = " The lifetime `'a` refers to the lifetime of the matcher that produced"] # [doc = " this glob."] # [derive (Clone , Debug)] # [allow (dead_code)] pub struct Glob < 'a > (GlobInner < 'a >) ;
};
}
