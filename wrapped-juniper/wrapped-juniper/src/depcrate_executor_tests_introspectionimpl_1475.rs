// Generated macro for impl_1475 (impl)
macro_rules! Depcrate_executor_tests_introspectionimpl_1475 {
() => {
// Module: crate::executor_tests::introspection
// Provides: {"impl_1475"}
// Dependencies: {}
# [doc = " The root query object in the schema"] # [graphql_object (impl = InterfaceValue)] impl Root { fn sample_enum () -> Sample { Sample :: One } # [doc = " A sample scalar field on the object"] fn sample_scalar (# [graphql (description = "The first number")] first : i32 , # [graphql (description = "The second number" , default = 123)] second : i32 ,) -> Scalar { Scalar (first + second) } }
};
}
