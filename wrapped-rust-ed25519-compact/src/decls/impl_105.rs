macro_rules! deps {
    () => {
        GeP3!();
        GeP2!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl From < GeP2 > for GeP3 { fn from (p : GeP2) -> GeP3 { GeP3 { x : p . x , y : p . y , z : p . z , t : p . x * p . y , } } }
    };
}

impl_105!()