macro_rules! spacing_to_external {
    () => {
        # [allow (unused)] fn spacing_to_external (spacing : Spacing) -> proc_macro :: Spacing { match spacing { Spacing :: Alone | Spacing :: JointHidden => proc_macro :: Spacing :: Alone , Spacing :: Joint => proc_macro :: Spacing :: Joint , } }
    };
}

spacing_to_external!();