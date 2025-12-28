macro_rules! deps {
    () => {
        Signature!();
        StashSaveOptions!();
        IntoCString!();
    };
}

macro_rules! impl_732 {
    () => {
        deps!();
        impl < 'a > StashSaveOptions < 'a > { # [doc = " Creates a default"] pub fn new (stasher : Signature < 'a >) -> Self { let mut opts = Self { message : None , flags : None , stasher , pathspec : Vec :: new () , pathspec_ptrs : Vec :: new () , raw_opts : unsafe { mem :: zeroed () } , } ; assert_eq ! (unsafe { raw :: git_stash_save_options_init (& mut opts . raw_opts , raw :: GIT_STASH_SAVE_OPTIONS_VERSION ,) } , 0) ; opts } # [doc = " Customize optional `flags` field"] pub fn flags (& mut self , flags : Option < StashFlags >) -> & mut Self { self . flags = flags ; self } # [doc = " Add to the array of paths patterns to build the stash."] pub fn pathspec < T : IntoCString > (& mut self , pathspec : T) -> & mut Self { let s = util :: cstring_to_repo_path (pathspec) . unwrap () ; self . pathspec_ptrs . push (s . as_ptr ()) ; self . pathspec . push (s) ; self } # [doc = " Acquire a pointer to the underlying raw options."] # [doc = ""] # [doc = " This function is unsafe as the pointer is only valid so long as this"] # [doc = " structure is not moved, modified, or used elsewhere."] pub unsafe fn raw (& mut self) -> * const raw :: git_stash_save_options { self . raw_opts . flags = self . flags . unwrap_or_else (StashFlags :: empty) . bits () as c_uint ; self . raw_opts . message = crate :: call :: convert (& self . message) ; self . raw_opts . paths . count = self . pathspec_ptrs . len () as size_t ; self . raw_opts . paths . strings = self . pathspec_ptrs . as_ptr () as * mut _ ; self . raw_opts . stasher = self . stasher . raw () ; & self . raw_opts as * const _ } }
    };
}

impl_732!()