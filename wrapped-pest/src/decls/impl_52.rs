macro_rules! deps {
    () => {
        RuleType!();
        Pair!();
        Pairs!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < 'i , R : RuleType > Iterator for Pairs < 'i , R > { type Item = Pair < 'i , R > ; fn next (& mut self) -> Option < Self :: Item > { let pair = self . peek () ? ; self . start = self . pair () + 1 ; self . pairs_count -= 1 ; Some (pair) } fn size_hint (& self) -> (usize , Option < usize >) { let len = < Self as ExactSizeIterator > :: len (self) ; (len , Some (len)) } }
    };
}

impl_52!()