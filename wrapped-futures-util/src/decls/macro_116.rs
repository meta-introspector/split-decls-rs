macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! macro_116 {
    () => {
        deps!();
        pin_project ! { # [project = TryFlattenProj] # [derive (Debug)] pub enum TryFlatten < Fut1 , Fut2 > { First { # [pin] f : Fut1 } , Second { # [pin] f : Fut2 } , Empty , } }
    };
}

macro_116!()