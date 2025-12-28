macro_rules! deps {
    () => {
        Axis!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl Axis { fn next (self) -> Option < Axis > { use crate :: Axis :: * ; match self { BottomX => Some (LeftY) , LeftY => Some (RightY) , RightY => Some (TopX) , TopX => None , } } }
    };
}

impl_40!()