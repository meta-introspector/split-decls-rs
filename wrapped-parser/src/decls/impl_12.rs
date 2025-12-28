macro_rules! deps {
    () => {
        ErrorPositionsInner!();
        ErrorPositions!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl DoubleEndedIterator for ErrorPositions { fn next_back (& mut self) -> Option < Self :: Item > { match self . 0 { ErrorPositionsInner :: Two (a , b) => { self . 0 = ErrorPositionsInner :: One (a) ; Some (b) } ErrorPositionsInner :: One (a) => { self . 0 = ErrorPositionsInner :: None ; Some (a) } ErrorPositionsInner :: None => None , } } }
    };
}

impl_12!()