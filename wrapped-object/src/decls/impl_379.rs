macro_rules! deps {
    () => {
        Item!();
        Crel!();
        Result!();
        CrelIterator!();
    };
}

macro_rules! impl_379 {
    () => {
        deps!();
        impl < 'data > Iterator for CrelIterator < 'data > { type Item = read :: Result < Crel > ; fn next (& mut self) -> Option < Self :: Item > { if self . state . index >= self . header . count { return None ; } let result = self . parse () ; if result . is_err () { self . state . index = self . header . count ; } Some (result) } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
    };
}

impl_379!();