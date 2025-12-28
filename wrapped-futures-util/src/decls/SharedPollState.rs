macro_rules! SharedPollState {
    () => {
        # [doc = " Internal polling state of the stream."] # [derive (Clone , Debug)] struct SharedPollState { state : Arc < AtomicU8 > , }
    };
}

SharedPollState!()