macro_rules! deps {
    () => {
        CacheValue!();
        CacheKey!();
        Options!();
        Pipeline!();
        Mode!();
    };
}

macro_rules! Platform {
    () => {
        deps!();
        # [doc = " A utility for performing a diff of two blobs, including flexible conversions, conversion-caching"] # [doc = " acquisition of diff information."] # [doc = " Note that this instance will not call external filters as their output can't be known programmatically,"] # [doc = " but it allows to prepare their input if the caller wishes to perform this task."] # [doc = ""] # [doc = " Optimized for NxM lookups with built-in caching."] # [derive (Clone)] pub struct Platform { # [doc = " The old version of a diff-able blob, if set."] old : Option < platform :: CacheKey > , # [doc = " The new version of a diff-able blob, if set."] new : Option < platform :: CacheKey > , # [doc = " Options to alter how diffs should be performed."] pub options : platform :: Options , # [doc = " A way to convert objects into a diff-able format."] pub filter : Pipeline , # [doc = " A way to access .gitattributes"] pub attr_stack : gix_worktree :: Stack , # [doc = " The way we convert resources into diffable states."] pub filter_mode : pipeline :: Mode , # [doc = " A continuously growing cache keeping ready-for-diff blobs by their path in the worktree,"] # [doc = " as that is what affects their final diff-able state."] # [doc = ""] # [doc = " That way, expensive rewrite-checks with NxM matrix checks would be as fast as possible,"] # [doc = " avoiding duplicate work."] diff_cache : HashMap < platform :: CacheKey , platform :: CacheValue > , # [doc = " A list of previously used buffers, ready for re-use."] free_list : Vec < Vec < u8 > > , }
    };
}

Platform!()