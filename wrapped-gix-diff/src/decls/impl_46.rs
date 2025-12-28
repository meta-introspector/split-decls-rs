macro_rules! deps {
    () => {
        Action!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl Action { # [doc = " Returns true if this action means to stop the traversal."] pub fn cancelled (& self) -> bool { matches ! (self , Action :: Cancel) } }
    };
}

impl_46!()