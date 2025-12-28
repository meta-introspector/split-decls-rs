macro_rules! spacing_to_internal {
    () => {
        # [allow (unused)] fn spacing_to_internal (spacing : proc_macro :: Spacing) -> Spacing { match spacing { proc_macro :: Spacing :: Alone => Spacing :: Alone , proc_macro :: Spacing :: Joint => Spacing :: Joint , } }
    };
}

spacing_to_internal!();