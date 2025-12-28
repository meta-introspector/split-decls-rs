macro_rules! deps {
    () => {
        CountItem!();
        CoalesceBy!();
        CoalescePredicate!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < I , F , C > FusedIterator for CoalesceBy < I , F , C > where I : Iterator , F : CoalescePredicate < I :: Item , C :: CItem > , C : CountItem < I :: Item > , { }
    };
}

impl_16!();