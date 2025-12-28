macro_rules! deps {
    () => {
        Error!();
        IntoCString!();
    };
}

macro_rules! set_extensions {
    () => {
        deps!();
        # [doc = " Set that the given git extensions are supported by the caller. Extensions"] # [doc = " supported by libgit2 may be negated by prefixing them with a `!`."] # [doc = " For example: setting extensions to `[ \"!noop\", \"newext\" ]` indicates that"] # [doc = " the caller does not want to support repositories with the `noop` extension"] # [doc = " but does want to support repositories with the `newext` extension."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " libgit2 stores user extensions in a static variable."] # [doc = " This function is effectively modifying a `static mut` and should be treated as such"] pub unsafe fn set_extensions < E > (extensions : & [E]) -> Result < () , Error > where for < 'x > & 'x E : IntoCString , { crate :: init () ; let extensions = extensions . iter () . map (| e | e . into_c_string ()) . collect :: < Result < Vec < _ > , _ > > () ? ; let extension_ptrs = extensions . iter () . map (| e | e . as_ptr ()) . collect :: < Vec < _ > > () ; try_call ! (raw :: git_libgit2_opts (raw :: GIT_OPT_SET_EXTENSIONS as libc :: c_int , extension_ptrs . as_ptr () , extension_ptrs . len () as libc :: size_t)) ; Ok (()) }
    };
}

set_extensions!();