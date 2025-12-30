// Generated macro for DataContext (trait)
macro_rules! Depcrate_contextDataContext {
() => {
// Module: crate::context
// Provides: {"DataContext"}
// Dependencies: {}
# [doc = " Data related functions of the context."] pub trait DataContext < 'a > { # [doc = " Gets the global data defined in the `Context` or `Schema`."] # [doc = ""] # [doc = " If both `Schema` and `Query` have the same data type, the data in the"] # [doc = " `Query` is obtained."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns a `Error` if the specified type data does not exist."] fn data < D : Any + Send + Sync > (& self) -> Result < & 'a D > ; # [doc = " Gets the global data defined in the `Context` or `Schema`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " It will panic if the specified data type does not exist."] fn data_unchecked < D : Any + Send + Sync > (& self) -> & 'a D ; # [doc = " Gets the global data defined in the `Context` or `Schema` or `None` if"] # [doc = " the specified type data does not exist."] fn data_opt < D : Any + Send + Sync > (& self) -> Option < & 'a D > ; }
};
}
