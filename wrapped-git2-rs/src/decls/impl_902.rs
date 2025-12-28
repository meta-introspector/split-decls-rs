macro_rules! impl_902 {
    () => {
        impl StashApplyFlags { is_bit_set ! (is_default , StashApplyFlags :: DEFAULT) ; is_bit_set ! (is_reinstate_index , StashApplyFlags :: REINSTATE_INDEX) ; }
    };
}

impl_902!();