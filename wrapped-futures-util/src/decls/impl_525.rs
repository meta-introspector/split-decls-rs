macro_rules! deps {
    () => {
        SharedPollState!();
        PollStateBomb!();
    };
}

macro_rules! impl_525 {
    () => {
        deps!();
        impl < 'a , F : FnOnce (& SharedPollState) -> u8 > PollStateBomb < 'a , F > { # [doc = " Constructs new bomb with the given state."] fn new (state : & 'a SharedPollState , drop : F) -> Self { Self { state , drop : Some (drop) } } # [doc = " Deactivates bomb, forces it to not call provided function when dropped."] fn deactivate (mut self) { self . drop . take () ; } }
    };
}

impl_525!()