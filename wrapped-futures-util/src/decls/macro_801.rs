macro_rules! deps {
    () => {
        InternalState!();
    };
}

macro_rules! macro_801 {
    () => {
        deps!();
        pin_project ! { # [doc = " Stream for the [`select_with_strategy()`] function. See function docs for details."] # [must_use = "streams do nothing unless polled"] # [project = SelectWithStrategyProj] pub struct SelectWithStrategy < St1 , St2 , Clos , State > { # [pin] stream1 : St1 , # [pin] stream2 : St2 , internal_state : InternalState , state : State , clos : Clos , } }
    };
}

macro_801!()