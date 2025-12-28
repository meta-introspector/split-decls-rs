macro_rules! deps {
    () => {
        Style!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Style { pub fn as_slice (self) -> & 'static str { match self { Style :: None => "" , Style :: Solid => "solid" , Style :: Dashed => "dashed" , Style :: Dotted => "dotted" , Style :: Bold => "bold" , Style :: Rounded => "rounded" , Style :: Diagonals => "diagonals" , Style :: Filled => "filled" , Style :: Striped => "striped" , Style :: Wedged => "wedged" , } } }
    };
}

impl_2!()