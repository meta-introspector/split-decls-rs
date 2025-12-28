macro_rules! deps {
    () => {
        RuleType!();
        FlatPairs!();
        Pair!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'i , R : RuleType > Iterator for FlatPairs < 'i , R > { type Item = Pair < 'i , R > ; fn next (& mut self) -> Option < Self :: Item > { if self . start >= self . end { return None ; } let pair = pair :: new (Rc :: clone (& self . queue) , self . input , Rc :: clone (& self . line_index) , self . start ,) ; self . next_start () ; Some (pair) } fn size_hint (& self) -> (usize , Option < usize >) { let len = < Self as ExactSizeIterator > :: len (self) ; (len , Some (len)) } }
    };
}

impl_26!()