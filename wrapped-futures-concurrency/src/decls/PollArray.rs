macro_rules! deps {
    () => {
        PollState!();
    };
}

macro_rules! PollArray {
    () => {
        deps!();
        pub (crate) struct PollArray < const N : usize > { state : [PollState ; N] , }
    };
}

PollArray!();