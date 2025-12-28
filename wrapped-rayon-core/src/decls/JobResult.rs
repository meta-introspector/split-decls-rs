macro_rules! JobResult {
    () => {
        pub (super) enum JobResult < T > { None , Ok (T) , Panic (Box < dyn Any + Send >) , }
    };
}

JobResult!();