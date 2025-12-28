macro_rules! deps {
    () => {
        Task!();
    };
}

macro_rules! Message {
    () => {
        deps!();
        enum Message { Run (Task) , Close , }
    };
}

Message!()