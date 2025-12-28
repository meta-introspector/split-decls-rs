macro_rules! TestIdGuard {
    () => {
        # [doc = " See [`init_root`]"] pub struct TestIdGuard { _private : () , }
    };
}

TestIdGuard!()