macro_rules! AssertNotLoaded {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_assert_not_loaded)] pub (crate) struct AssertNotLoaded ;
    };
}

AssertNotLoaded!()