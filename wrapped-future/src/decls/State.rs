macro_rules! State {
    () => {
        struct State < T : Async > { result : Option < Result < T :: Output > > , completed : Option < T :: CompletedHandler > , completed_assigned : bool , }
    };
}

State!();