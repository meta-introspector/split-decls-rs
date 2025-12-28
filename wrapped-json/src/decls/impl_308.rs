macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl < T : Into < Value > , const N : usize > From < [T ; N] > for Value { fn from (array : [T ; N]) -> Self { Value :: Array (array . into_iter () . map (Into :: into) . collect ()) } }
    };
}

impl_308!();