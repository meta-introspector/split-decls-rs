macro_rules! deps {
    () => {
        FindMatches!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        impl < 'r , 'h > core :: iter :: FusedIterator for FindMatches < 'r , 'h > { }
    };
}

impl_346!();