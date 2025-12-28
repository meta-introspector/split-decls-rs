macro_rules! deps {
    () => {
        Pos!();
        ErrorPositions!();
        ErrorPositionsInner!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl Iterator for ErrorPositions { type Item = Pos ; fn next (& mut self) -> Option < Self :: Item > { match self . 0 { ErrorPositionsInner :: Two (a , b) => { self . 0 = ErrorPositionsInner :: One (b) ; Some (a) } ErrorPositionsInner :: One (a) => { self . 0 = ErrorPositionsInner :: None ; Some (a) } ErrorPositionsInner :: None => None , } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
    };
}

impl_142!()