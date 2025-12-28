macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! CharLit {
    () => {
        deps!();
        # [doc = " A character literal, e.g. `'g'` or `'🦊'`."] # [doc = ""] # [doc = " See [the reference][ref] for more information."] # [doc = ""] # [doc = " [ref]: https://doc.rust-lang.org/reference/tokens.html#character-literals"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct CharLit < B : Buffer > { raw : B , # [doc = " Start index of the suffix or `raw.len()` if there is no suffix."] start_suffix : usize , value : char , }
    };
}

CharLit!();