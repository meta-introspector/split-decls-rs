macro_rules! deps {
    () => {
        PollTimeout!();
        Error!();
        Result!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        impl TryFrom < PollTimeout > for Duration { type Error = () ; fn try_from (x : PollTimeout) -> std :: result :: Result < Self , () > { x . duration () . ok_or (()) } }
    };
}

impl_260!()