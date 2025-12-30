// Generated macro for ALLOC_NOT_ENABLED_ERROR (const)
macro_rules! Depcrate_builderALLOC_NOT_ENABLED_ERROR {
() => {
// Module: crate::builder
// Provides: {"ALLOC_NOT_ENABLED_ERROR"}
// Dependencies: {}
const ALLOC_NOT_ENABLED_ERROR : & str = r#"`alloc` is disabled within 'derive_builder', consider one of the following:
* enable feature `alloc` on 'derive_builder' if a `global_allocator` is present
* use a custom error `#[builder(build_fn(error = "path::to::Error"))]
* disable the validation error `#[builder(build_fn(error(validation_error = false)))]"# ;
};
}
