macro_rules! deps {
    () => {
        RuleType!();
        Pairs!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < R : RuleType > DoubleEndedIterator for Pairs < '_ , R > { fn next_back (& mut self) -> Option < Self :: Item > { if self . end <= self . start { return None ; } self . end = self . pair_from_end () ; self . pairs_count -= 1 ; let pair = pair :: new (Rc :: clone (& self . queue) , self . input , Rc :: clone (& self . line_index) , self . end ,) ; Some (pair) } }
    };
}

impl_53!();