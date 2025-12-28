macro_rules! is_ci {
    () => {
        # [doc = " Whether or not this running in a Continuous Integration environment."] fn is_ci () -> bool { option_env ! ("CI") . is_some () || option_env ! ("TF_BUILD") . is_some () }
    };
}

is_ci!()