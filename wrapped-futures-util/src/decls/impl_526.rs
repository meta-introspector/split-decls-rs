macro_rules! deps {
    () => {
        PollStateBomb!();
        SharedPollState!();
    };
}

macro_rules! impl_526 {
    () => {
        deps!();
        impl < F : FnOnce (& SharedPollState) -> u8 > Drop for PollStateBomb < '_ , F > { fn drop (& mut self) { if let Some (drop) = self . drop . take () { (drop) (self . state) ; } } }
    };
}

impl_526!()