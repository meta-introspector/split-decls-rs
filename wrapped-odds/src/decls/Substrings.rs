macro_rules! deps {
    () => {
        Suffixes!();
        Prefixes!();
    };
}

macro_rules! Substrings {
    () => {
        deps!();
        # [doc = " Iterator of all non-empty substrings"] # [derive (Clone)] pub struct Substrings < 'a > { iter : iter :: FlatMap < Prefixes < 'a > , Suffixes < 'a > , fn (& 'a str) -> Suffixes < 'a > > , }
    };
}

Substrings!();