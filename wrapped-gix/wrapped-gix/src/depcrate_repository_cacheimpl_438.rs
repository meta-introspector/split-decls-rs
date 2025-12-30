// Generated macro for impl_438 (impl)
macro_rules! Depcrate_repository_cacheimpl_438 {
() => {
// Module: crate::repository::cache
// Provides: {"impl_438"}
// Dependencies: {}
# [doc = " Handling of InMemory object writing"] impl crate :: Repository { # [doc = " When writing objects, keep them in memory instead of writing them to disk."] # [doc = " This makes any change to the object database non-persisting, while keeping the view"] # [doc = " to the object database consistent for this instance."] pub fn with_object_memory (mut self) -> Self { self . objects . enable_object_memory () ; self } }
};
}
