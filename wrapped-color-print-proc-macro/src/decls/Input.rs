macro_rules! Input {
    () => {
        pub type Input < 'a > = & 'a str ;
    };
}

Input!();