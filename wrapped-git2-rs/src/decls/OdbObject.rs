macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! OdbObject {
    () => {
        deps!();
        # [doc = " An object from the Object Database."] pub struct OdbObject < 'a > { raw : * mut raw :: git_odb_object , _marker : marker :: PhantomData < Object < 'a > > , }
    };
}

OdbObject!()