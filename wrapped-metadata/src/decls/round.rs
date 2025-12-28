macro_rules! round {
    () => {
        pub fn round (size : usize , round : usize) -> usize { let round = round - 1 ; (size + round) & ! round }
    };
}

round!();