macro_rules! deps {
    () => {
        HelpUseLatestEdition!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl HelpUseLatestEdition { pub (crate) fn new () -> Self { let edition = LATEST_STABLE_EDITION ; if rustc_session :: utils :: was_invoked_from_cargo () { Self :: Cargo { edition } } else { Self :: Standalone { edition } } } }
    };
}

impl_87!()