macro_rules! deps {
    () => {
        PollNext!();
        Select!();
    };
}

macro_rules! macro_790 {
    () => {
        deps!();
        pin_project ! { # [doc = " Stream for the [`select()`] function."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Select < St1 , St2 > { # [pin] inner : SelectWithStrategy < St1 , St2 , fn (& mut PollNext) -> PollNext , PollNext >, } }
    };
}

macro_790!()