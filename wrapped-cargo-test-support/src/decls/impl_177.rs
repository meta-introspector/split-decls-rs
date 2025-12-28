macro_rules! deps {
    () => {
        ChannelChangerCommandExt!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl ChannelChangerCommandExt for snapbox :: cmd :: Command { fn masquerade_as_nightly_cargo (self , _reasons : & [& str]) -> Self { self . env ("__CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS" , "nightly") } }
    };
}

impl_177!()