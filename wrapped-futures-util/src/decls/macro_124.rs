macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! macro_124 {
    () => {
        deps!();
        pin_project ! { # [project = TryFlattenErrProj] # [derive (Debug)] pub enum TryFlattenErr < Fut1 , Fut2 > { First { # [pin] f : Fut1 } , Second { # [pin] f : Fut2 } , Empty , } }
    };
}

macro_124!()