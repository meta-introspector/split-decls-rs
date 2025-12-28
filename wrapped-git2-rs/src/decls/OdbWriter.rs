macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! OdbWriter {
    () => {
        deps!();
        # [doc = " A structure to represent a git ODB wstream"] pub struct OdbWriter < 'repo > { raw : * mut raw :: git_odb_stream , _marker : marker :: PhantomData < Object < 'repo > > , }
    };
}

OdbWriter!()