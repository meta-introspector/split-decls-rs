macro_rules! AssertLoaded {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_assert_loaded)] pub (crate) struct AssertLoaded ;
    };
}

AssertLoaded!();