macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! macro_38 {
    () => {
        deps!();
        pin_project ! { # [project = FlattenProj] # [derive (Debug)] pub enum Flatten < Fut1 , Fut2 > { First { # [pin] f : Fut1 } , Second { # [pin] f : Fut2 } , Empty , } }
    };
}

macro_38!()