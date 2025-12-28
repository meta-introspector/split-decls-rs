macro_rules! State {
    () => {
        enum State { PendingEnter , Normal , PendingExit , }
    };
}

State!();