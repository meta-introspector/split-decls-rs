macro_rules! deps {
    () => {
        InvariantOpaque!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        # [repr (C)] pub (crate) struct Builder < 'a > (InvariantOpaque < 'a >) ;
    };
}

Builder!()