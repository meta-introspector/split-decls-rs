macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! OdbReader {
    () => {
        deps!();
        # [doc = " A structure to represent a git ODB rstream"] pub struct OdbReader < 'repo > { raw : * mut raw :: git_odb_stream , _marker : marker :: PhantomData < Object < 'repo > > , }
    };
}

OdbReader!()