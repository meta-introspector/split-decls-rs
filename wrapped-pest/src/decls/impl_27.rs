macro_rules! deps {
    () => {
        RuleType!();
        FlatPairs!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < R : RuleType > DoubleEndedIterator for FlatPairs < '_ , R > { fn next_back (& mut self) -> Option < Self :: Item > { if self . end <= self . start { return None ; } self . next_start_from_end () ; let pair = pair :: new (Rc :: clone (& self . queue) , self . input , Rc :: clone (& self . line_index) , self . end ,) ; Some (pair) } }
    };
}

impl_27!()