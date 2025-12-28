macro_rules! ENV_CERT_DIR {
    () => {
        # [doc = " The OpenSSL environment variable to configure what certificates directory to use."] pub const ENV_CERT_DIR : & 'static str = "SSL_CERT_DIR" ;
    };
}

ENV_CERT_DIR!();