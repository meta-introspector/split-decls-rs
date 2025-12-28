macro_rules! impl_30 {
    () => {
        impl MergePreference { is_bit_set ! (is_none , MergePreference :: NONE) ; is_bit_set ! (is_no_fast_forward , MergePreference :: NO_FAST_FORWARD) ; is_bit_set ! (is_fastforward_only , MergePreference :: FASTFORWARD_ONLY) ; }
    };
}

impl_30!()