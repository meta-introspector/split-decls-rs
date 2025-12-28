macro_rules! deps {
    () => {
        DedupPred2CoalescePred!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < DP > fmt :: Debug for DedupPred2CoalescePred < DP > { debug_fmt_fields ! (DedupPred2CoalescePred ,) ; }
    };
}

impl_27!()