macro_rules! InternalState {
    () => {
        enum InternalState { Start , LeftFinished , RightFinished , BothFinished , }
    };
}

InternalState!()