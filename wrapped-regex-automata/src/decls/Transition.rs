macro_rules! deps {
    () => {
        StateID!();
    };
}

macro_rules! Transition {
    () => {
        deps!();
        # [doc = " A transition is a single range of bytes. If a particular byte is in this"] # [doc = " range, then the corresponding machine may transition to the state pointed"] # [doc = " to by `next_id`."] # [derive (Clone)] struct Transition { # [doc = " The byte range."] range : Utf8Range , # [doc = " The next state to transition to."] next_id : StateID , }
    };
}

Transition!()