macro_rules! deps {
    () => {
        CoalesceBy!();
        CoalescePredicate!();
        CountItem!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < I , F , C > FusedIterator for CoalesceBy < I , F , C > where I : Iterator , F : CoalescePredicate < I :: Item , C :: CItem > , C : CountItem < I :: Item > , { }
    };
}

impl_16!()