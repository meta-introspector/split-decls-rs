macro_rules! deps {
    () => {
        ErrorPositions!();
        ErrorPositionsInner!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl ExactSizeIterator for ErrorPositions { fn len (& self) -> usize { match self . 0 { ErrorPositionsInner :: Two (_ , _) => 2 , ErrorPositionsInner :: One (_) => 1 , ErrorPositionsInner :: None => 0 , } } }
    };
}

impl_145!()