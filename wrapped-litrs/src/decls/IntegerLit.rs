macro_rules! deps {
    () => {
        Buffer!();
        IntegerBase!();
    };
}

macro_rules! IntegerLit {
    () => {
        deps!();
        # [doc = " An integer literal, e.g. `27`, `0x7F`, `0b101010u8` or `5_000_000i64`."] # [doc = ""] # [doc = " An integer literal consists of an optional base prefix (`0b`, `0o`, `0x`),"] # [doc = " the main part (digits and underscores), and an optional type suffix"] # [doc = " (e.g. `u64` or `i8`). See [the reference][ref] for more information."] # [doc = ""] # [doc = " Note that integer literals are always positive: the grammar does not contain"] # [doc = " the minus sign at all. The minus sign is just the unary negate operator,"] # [doc = " not part of the literal. Which is interesting for cases like `- 128i8`:"] # [doc = " here, the literal itself would overflow the specified type (`i8` cannot"] # [doc = " represent 128). That's why in rustc, the literal overflow check is"] # [doc = " performed as a lint after parsing, not during the lexing stage. Similarly,"] # [doc = " [`IntegerLit::parse`] does not perform an overflow check."] # [doc = ""] # [doc = " [ref]: https://doc.rust-lang.org/reference/tokens.html#integer-literals"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] # [non_exhaustive] pub struct IntegerLit < B : Buffer > { # [doc = " The raw literal. Grammar: `<prefix?><main part><suffix?>`."] raw : B , # [doc = " First index of the main number part (after the base prefix)."] start_main_part : usize , # [doc = " First index not part of the main number part."] end_main_part : usize , # [doc = " Parsed `raw[..start_main_part]`."] base : IntegerBase , }
    };
}

IntegerLit!();