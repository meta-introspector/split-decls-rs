macro_rules! deps {
    () => {
        Error!();
        IntoCString!();
    };
}

macro_rules! set_ssl_cert_dir {
    () => {
        deps!();
        # [doc = " Set the SSL certificate-authority location to `path`. `path` is the location"] # [doc = " of a directory holding several certificates, one per file."] pub unsafe fn set_ssl_cert_dir < P > (path : P) -> Result < () , Error > where P : IntoCString , { crate :: init () ; unsafe { try_call ! (raw :: git_libgit2_opts (raw :: GIT_OPT_SET_SSL_CERT_LOCATIONS as libc :: c_int , core :: ptr :: null ::< libc :: c_char > () , path . into_c_string () ?. as_ptr ())) ; } Ok (()) }
    };
}

set_ssl_cert_dir!()