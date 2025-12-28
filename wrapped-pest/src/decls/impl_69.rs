macro_rules! deps {
    () => {
        Tokens!();
        RuleType!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < R : RuleType > DoubleEndedIterator for Tokens < '_ , R > { fn next_back (& mut self) -> Option < Self :: Item > { if self . end <= self . start { return None ; } let token = self . create_token (self . end - 1) ; self . end -= 1 ; Some (token) } }
    };
}

impl_69!()