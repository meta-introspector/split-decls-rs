macro_rules! deps {
    () => {
        Grid!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl Grid { fn next (self) -> Option < Grid > { use crate :: Grid :: * ; match self { Major => Some (Minor) , Minor => None , } } }
    };
}

impl_43!()