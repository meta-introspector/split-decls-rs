macro_rules! deps {
    () => {
        IntoCString!();
        Error!();
    };
}

macro_rules! set_ssl_cert_file {
    () => {
        deps!();
        # [doc = " Set the SSL certificate-authority location to `file`. `file` is the location"] # [doc = " of a file containing several certificates concatenated together."] pub unsafe fn set_ssl_cert_file < P > (file : P) -> Result < () , Error > where P : IntoCString , { crate :: init () ; unsafe { try_call ! (raw :: git_libgit2_opts (raw :: GIT_OPT_SET_SSL_CERT_LOCATIONS as libc :: c_int , file . into_c_string () ?. as_ptr () , core :: ptr :: null ::< libc :: c_char > ())) ; } Ok (()) }
    };
}

set_ssl_cert_file!()