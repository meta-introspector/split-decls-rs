macro_rules! deps {
    () => {
        CertX509!();
        CertHostkey!();
    };
}

macro_rules! Cert {
    () => {
        deps!();
        # [doc = " A certificate for a remote connection, viewable as one of `CertHostkey` or"] # [doc = " `CertX509` currently."] pub struct Cert < 'a > { raw : * mut raw :: git_cert , _marker : marker :: PhantomData < & 'a raw :: git_cert > , }
    };
}

Cert!()