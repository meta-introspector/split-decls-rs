macro_rules! deps {
    () => {
        ReversedEdgeReference!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < R > ReversedEdgeReference < R > { # [doc = " Return the original, unreversed edge reference."] pub fn as_unreversed (& self) -> & R { & self . 0 } # [doc = " Consume `self` and return the original, unreversed edge reference."] pub fn into_unreversed (self) -> R { self . 0 } }
    };
}

impl_180!();