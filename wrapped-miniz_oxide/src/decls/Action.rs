macro_rules! deps {
    () => {
        State!();
        TINFLStatus!();
    };
}

macro_rules! Action {
    () => {
        deps!();
        enum Action { None , Jump (State) , End (TINFLStatus) , }
    };
}

Action!();