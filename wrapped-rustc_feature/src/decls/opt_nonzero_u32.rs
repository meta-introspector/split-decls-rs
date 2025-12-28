macro_rules! opt_nonzero_u32 {
    () => {
        macro_rules ! opt_nonzero_u32 { () => { None } ; ($ val : expr) => { Some (NonZeroU32 :: new ($ val) . unwrap ()) } ; }
    };
}

opt_nonzero_u32!()