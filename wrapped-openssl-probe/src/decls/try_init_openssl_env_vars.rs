macro_rules! deps {
    () => {
        ProbeResult!();
    };
}

macro_rules! try_init_openssl_env_vars {
    () => {
        deps!();
        # [doc = " Probe for SSL certificates on the system, then configure the SSL certificate `SSL_CERT_FILE`"] # [doc = " and `SSL_CERT_DIR` environment variables in this process for OpenSSL to use."] # [doc = ""] # [doc = " Preconfigured values in the environment variables will not be overwritten if the paths they"] # [doc = " point to exist and are accessible."] # [doc = ""] # [doc = " Returns `true` if any certificate file or directory was found while probing."] # [doc = " Combine this with `has_ssl_cert_env_vars()` to check whether previously configured environment"] # [doc = " variables are valid."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is not safe because it mutates the process's environment"] # [doc = " variables which is generally not safe. See the [documentation in libstd][doc]"] # [doc = " for information about why setting environment variables is not safe."] # [doc = ""] # [doc = " If possible use the [`probe`] function and directly configure OpenSSL"] # [doc = " methods instead of relying on environment variables."] # [doc = ""] # [doc = " [doc]: https://doc.rust-lang.org/stable/std/env/fn.set_var.html#safety"] pub unsafe fn try_init_openssl_env_vars () -> bool { let ProbeResult { cert_file , cert_dir , } = probe () ; if let Some (path) = & cert_file { unsafe { put (ENV_CERT_FILE , path) ; } } if let Some (path) = & cert_dir { unsafe { put (ENV_CERT_DIR , path) ; } } unsafe fn put (var : & str , path : & Path) { if env :: var_os (var) . as_deref () != Some (path . as_os_str ()) { unsafe { env :: set_var (var , path) ; } } } cert_file . is_some () || cert_dir . is_some () }
    };
}

try_init_openssl_env_vars!();