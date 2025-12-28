macro_rules! CertX509 {
    () => {
        # [doc = " X.509 certificate information"] pub struct CertX509 < 'a > { raw : * mut raw :: git_cert_x509 , _marker : marker :: PhantomData < & 'a raw :: git_cert > , }
    };
}

CertX509!();