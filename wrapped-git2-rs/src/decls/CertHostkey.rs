macro_rules! CertHostkey {
    () => {
        # [doc = " Hostkey information taken from libssh2"] pub struct CertHostkey < 'a > { raw : * mut raw :: git_cert_hostkey , _marker : marker :: PhantomData < & 'a raw :: git_cert > , }
    };
}

CertHostkey!()