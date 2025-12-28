macro_rules! deps {
    () => {
        GeP1P1!();
        GeP2!();
        GeP3!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl GeP1P1 { fn to_p2 (& self) -> GeP2 { GeP2 { x : self . x * self . t , y : self . y * self . z , z : self . z * self . t , } } fn to_p3 (& self) -> GeP3 { GeP3 { x : self . x * self . t , y : self . y * self . z , z : self . z * self . t , t : self . x * self . y , } } }
    };
}

impl_104!()