macro_rules! deps {
    () => {
        DelayedFilteredStream!();
    };
}

macro_rules! Outcome {
    () => {
        deps!();
        pub enum Outcome < 'a > { # [doc = " The file was written."] Written { # [doc = " The amount of written bytes."] bytes : usize , } , # [doc = " The will be ready later."] Delayed (DelayedFilteredStream < 'a >) , }
    };
}

Outcome!()