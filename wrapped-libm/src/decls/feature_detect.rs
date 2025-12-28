macro_rules! feature_detect {
    () => {
        # [cfg (target_has_atomic = "ptr")] pub (crate) mod feature_detect ;
    };
}

feature_detect!();