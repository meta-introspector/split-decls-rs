macro_rules! deps {
    () => {
        SharedPollState!();
    };
}

macro_rules! PollStateBomb {
    () => {
        deps!();
        # [doc = " Used to execute some function on the given state when dropped."] struct PollStateBomb < 'a , F : FnOnce (& SharedPollState) -> u8 > { state : & 'a SharedPollState , drop : Option < F > , }
    };
}

PollStateBomb!()