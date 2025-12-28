macro_rules! deps {
    () => {
        DiffPatchidOptions!();
    };
}

macro_rules! impl_363 {
    () => {
        deps!();
        impl DiffPatchidOptions { # [doc = " Creates a new set of patchid options,"] # [doc = " initialized to the default values"] pub fn new () -> Self { let mut opts = DiffPatchidOptions { raw : unsafe { mem :: zeroed () } , } ; assert_eq ! (unsafe { raw :: git_diff_patchid_options_init (& mut opts . raw , raw :: GIT_DIFF_PATCHID_OPTIONS_VERSION ,) } , 0) ; opts } }
    };
}

impl_363!()