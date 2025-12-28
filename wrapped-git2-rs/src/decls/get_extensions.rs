macro_rules! deps {
    () => {
        Error!();
        StringArray!();
    };
}

macro_rules! get_extensions {
    () => {
        deps!();
        # [doc = " Returns the list of git extensions that are supported. This is the list of"] # [doc = " built-in extensions supported by libgit2 and custom extensions that have"] # [doc = " been added with [`set_extensions`]. Extensions that have been negated will"] # [doc = " not be returned."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " libgit2 stores user extensions in a static variable."] # [doc = " This function is effectively reading a `static mut` and should be treated as such"] pub unsafe fn get_extensions () -> Result < StringArray , Error > { crate :: init () ; let mut extensions = raw :: git_strarray { strings : ptr :: null_mut () , count : 0 , } ; try_call ! (raw :: git_libgit2_opts (raw :: GIT_OPT_GET_EXTENSIONS as libc :: c_int , & mut extensions)) ; Ok (StringArray :: from_raw (extensions)) }
    };
}

get_extensions!()