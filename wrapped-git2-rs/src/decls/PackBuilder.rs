macro_rules! deps {
    () => {
        Repository!();
        ProgressCb!();
    };
}

macro_rules! PackBuilder {
    () => {
        deps!();
        # [doc = " A builder for creating a packfile"] pub struct PackBuilder < 'repo > { raw : * mut raw :: git_packbuilder , _progress : Option < Box < Box < ProgressCb < 'repo > > > > , _marker : marker :: PhantomData < & 'repo Repository > , }
    };
}

PackBuilder!()