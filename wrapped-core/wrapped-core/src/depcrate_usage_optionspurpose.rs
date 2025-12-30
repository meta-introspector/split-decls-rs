// Generated macro for Purpose (enum)
macro_rules! Depcrate_usage_optionsPurpose {
() => {
// Module: crate::usage::options
// Provides: {"Purpose"}
// Dependencies: {}
# [doc = " The goal of tracing generic parameter usage."] # [doc = ""] # [doc = " Not all uses of type parameters imply a need to add bounds to a generated trait impl."] # [doc = " For example, a field of type `<Vec<T> as a::b::Trait>::Associated` does not need a"] # [doc = " `where T: Serialize` bound in `serde`."] # [doc = " However, a proc macro that is attempting to generate a helper struct _would_ need to"] # [doc = " know about this usage, or else the generated code would reference an unknown type `T`"] # [doc = " and fail to compile."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum Purpose { # [doc = " The tracing is being used to generate an `impl` block."] # [doc = ""] # [doc = " Uses such as `syn::TypePath.qself` will _not_ be returned."] BoundImpl , # [doc = " The tracing is being used to generate a new struct or enum."] # [doc = ""] # [doc = " All uses will be returned."] Declare , }
};
}
