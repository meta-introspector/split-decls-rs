// Generated macro for Options (struct)
macro_rules! Depcrate_cache_delta_traverseOptions {
() => {
// Module: crate::cache::delta::traverse
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options for [`Tree::traverse()`]."] pub struct Options < 'a , 's > { # [doc = " is a progress instance to track progress for each object in the traversal."] pub object_progress : Box < dyn DynNestedProgress > , # [doc = " is a progress instance to track the overall progress."] pub size_progress : & 's mut dyn Progress , # [doc = " If `Some`, only use the given number of threads. Otherwise, the number of threads to use will be selected based on"] # [doc = " the number of available logical cores."] pub thread_limit : Option < usize > , # [doc = " Abort the operation if the value is `true`."] pub should_interrupt : & 'a AtomicBool , # [doc = " specifies what kind of hashes we expect to be stored in oid-delta entries, which is viable to decoding them"] # [doc = " with the correct size."] pub object_hash : gix_hash :: Kind , }
};
}
