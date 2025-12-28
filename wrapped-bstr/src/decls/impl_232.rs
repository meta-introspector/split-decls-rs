macro_rules! deps {
    () => {
        Utf8Chunk!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl < 'a > Utf8Chunk < 'a > { # [doc = " Returns the (possibly empty) valid UTF-8 bytes in this chunk."] # [doc = ""] # [doc = " This may be empty if there are consecutive sequences of invalid UTF-8"] # [doc = " bytes."] # [inline] pub fn valid (& self) -> & 'a str { self . valid } # [doc = " Returns the (possibly empty) invalid UTF-8 bytes in this chunk that"] # [doc = " immediately follow the valid UTF-8 bytes in this chunk."] # [doc = ""] # [doc = " This is only empty when this chunk corresponds to the last chunk in"] # [doc = " the original bytes."] # [doc = ""] # [doc = " The maximum length of this slice is 3. That is, invalid UTF-8 byte"] # [doc = " sequences greater than 1 always correspond to a valid _prefix_ of"] # [doc = " a valid UTF-8 encoded codepoint. This corresponds to the \"substitution"] # [doc = " of maximal subparts\" strategy that is described in more detail in the"] # [doc = " docs for the"] # [doc = " [`ByteSlice::to_str_lossy`](trait.ByteSlice.html#method.to_str_lossy)"] # [doc = " method."] # [inline] pub fn invalid (& self) -> & 'a [u8] { self . invalid . as_bytes () } # [doc = " Returns whether the invalid sequence might still become valid if more"] # [doc = " bytes are added."] # [doc = ""] # [doc = " Returns true if the end of the input was reached unexpectedly,"] # [doc = " without encountering an unexpected byte."] # [doc = ""] # [doc = " This can only be the case for the last chunk."] # [inline] pub fn incomplete (& self) -> bool { self . incomplete } }
    };
}

impl_232!()