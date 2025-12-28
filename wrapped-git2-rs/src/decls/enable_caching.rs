macro_rules! enable_caching {
    () => {
        # [doc = " Controls whether or not libgit2 will cache loaded objects.  Enabled by"] # [doc = " default, but disabling this can improve performance and memory usage if"] # [doc = " loading a large number of objects that will not be referenced again."] # [doc = " Disabling this will cause repository objects to clear their caches when next"] # [doc = " accessed."] pub fn enable_caching (enabled : bool) { crate :: init () ; let error = unsafe { raw :: git_libgit2_opts (raw :: GIT_OPT_ENABLE_CACHING as libc :: c_int , enabled as libc :: c_int ,) } ; debug_assert ! (error >= 0) ; }
    };
}

enable_caching!()