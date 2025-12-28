macro_rules! deps {
    () => {
        Position!();
        LineColLocation!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl From < Position < '_ > > for LineColLocation { fn from (value : Position < '_ >) -> Self { Self :: Pos (value . line_col ()) } }
    };
}

impl_8!();