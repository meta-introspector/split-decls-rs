macro_rules! Delimited {
    () => {
        pub struct Delimited < I : Iterator > { is_first : bool , iter : Peekable < I > , }
    };
}

Delimited!()