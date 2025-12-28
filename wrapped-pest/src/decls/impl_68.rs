macro_rules! deps {
    () => {
        Tokens!();
        Token!();
        RuleType!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < 'i , R : RuleType > Iterator for Tokens < 'i , R > { type Item = Token < 'i , R > ; fn next (& mut self) -> Option < Self :: Item > { if self . start >= self . end { return None ; } let token = self . create_token (self . start) ; self . start += 1 ; Some (token) } fn size_hint (& self) -> (usize , Option < usize >) { let len = < Self as ExactSizeIterator > :: len (self) ; (len , Some (len)) } }
    };
}

impl_68!();