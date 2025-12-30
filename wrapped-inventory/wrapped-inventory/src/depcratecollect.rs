// Generated macro for collect (macro)
macro_rules! Depcratecollect {
() => {
// Module: crate
// Provides: {"collect"}
// Dependencies: {}
# [doc = " Associate a plugin registry with the specified type."] # [doc = ""] # [doc = " This call must be in the same crate that defines the plugin type. This macro"] # [doc = " does not \"run\" anything so place it outside of any function body."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Suppose we are writing a command line flags library and want to allow any"] # [doc = " source file in the application to register command line flags that are"] # [doc = " relevant to it."] # [doc = ""] # [doc = " This is the flag registration style used by [gflags] and is better suited"] # [doc = " for large scale development than maintaining a single central list of flags,"] # [doc = " as the central list would become an endless source of merge conflicts."] # [doc = ""] # [doc = " [gflags]: https://gflags.github.io/gflags/"] # [doc = ""] # [doc = " ```"] # [doc = " pub struct Flag {"] # [doc = "     short: char,"] # [doc = "     name: &'static str,"] # [doc = "     /* ... */"] # [doc = " }"] # [doc = ""] # [doc = " inventory::collect!(Flag);"] # [doc = " ```"] # [doc = ""] # [doc = " Refer to the [crate level documentation](index.html) for a complete example"] # [doc = " of submitting plugins and iterating a plugin registry."] # [macro_export] macro_rules ! collect { ($ ty : ty) => { impl $ crate :: Collect for $ ty { # [inline] fn registry () -> &'static $ crate :: Registry { static REGISTRY : $ crate :: Registry = $ crate :: Registry :: new () ; & REGISTRY } } } ; }
};
}
