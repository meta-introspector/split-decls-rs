macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! ByteLit {
    () => {
        deps!();
        # [doc = " A (single) byte literal, e.g. `b'k'` or `b'!'`."] # [doc = ""] # [doc = " See [the reference][ref] for more information."] # [doc = ""] # [doc = " [ref]: https://doc.rust-lang.org/reference/tokens.html#byte-literals"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct ByteLit < B : Buffer > { raw : B , # [doc = " Start index of the suffix or `raw.len()` if there is no suffix."] start_suffix : usize , value : u8 , }
    };
}

ByteLit!()