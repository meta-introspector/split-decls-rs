macro_rules! AssertKinds {
    () => {
        # [allow (dead_code)] trait AssertKinds : Send + Sync + Clone { }
    };
}

AssertKinds!();