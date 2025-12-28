macro_rules! next_prev {
    () => {
        pub (crate) fn next_prev () -> impl Iterator < Item = Direction > { [Direction :: Next , Direction :: Prev] . into_iter () }
    };
}

next_prev!()