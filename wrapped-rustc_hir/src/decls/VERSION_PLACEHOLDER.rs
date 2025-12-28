macro_rules! VERSION_PLACEHOLDER {
    () => {
        # [doc = " The version placeholder that recently stabilized features contain inside the"] # [doc = " `since` field of the `#[stable]` attribute."] # [doc = ""] # [doc = " For more, see [this pull request](https://github.com/rust-lang/rust/pull/100591)."] pub const VERSION_PLACEHOLDER : & str = concat ! ("CURRENT_RUSTC_VERSIO" , "N") ;
    };
}

VERSION_PLACEHOLDER!()