macro_rules! PositionCalculator {
    () => {
        pub (crate) struct PositionCalculator < 'a > { input : & 'a str , pos : usize , line : usize , column : usize , }
    };
}

PositionCalculator!();