macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! func_from_slice {
    () => {
        deps!();
        # [doc = " Macro to implement a `from_slice()` function. Returns `UnknownCryptoError`"] # [doc = " if the slice length is not accepted."] # [doc = " $lower_bound and $upper_bound is the inclusive range of which a slice might"] # [doc = " be acceptable in length. If a slice may only be a fixed size, $lower_bound"] # [doc = " and $upper_bound should be the same. The `value` field will always be allocated with"] # [doc = " a size of $upper_bound."] macro_rules ! func_from_slice (($ name : ident , $ lower_bound : expr , $ upper_bound : expr) => (# [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Construct from a given byte slice."] pub fn from_slice (slice : & [u8]) -> Result <$ name , UnknownCryptoError > { let slice_len = slice . len () ; if ! ($ lower_bound ..=$ upper_bound) . contains (& slice_len) { return Err (UnknownCryptoError) ; } let mut value = [0u8 ; $ upper_bound] ; value [.. slice_len] . copy_from_slice (slice) ; Ok ($ name { value , original_length : slice_len }) })) ;
    };
}

func_from_slice!();