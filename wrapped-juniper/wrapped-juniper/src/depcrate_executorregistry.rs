// Generated macro for Registry (struct)
macro_rules! Depcrate_executorRegistry {
() => {
// Module: crate::executor
// Provides: {"Registry"}
// Dependencies: {}
# [doc = " A type registry used to build schemas"] # [doc = ""] # [doc = " The registry gathers metadata for all types in a schema. It provides"] # [doc = " convenience methods to convert types implementing the `GraphQLType` trait"] # [doc = " into `Type` instances and automatically registers them."] pub struct Registry < S = DefaultScalarValue > { # [doc = " Currently registered types"] pub types : FnvHashMap < Name , MetaType < S > > , }
};
}
