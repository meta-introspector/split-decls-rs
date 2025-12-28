macro_rules! deps {
    () => {
        Result!();
        PollTimeout!();
        Error!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        impl TryFrom < PollTimeout > for Duration { type Error = () ; fn try_from (x : PollTimeout) -> std :: result :: Result < Self , () > { x . duration () . ok_or (()) } }
    };
}

impl_260!();