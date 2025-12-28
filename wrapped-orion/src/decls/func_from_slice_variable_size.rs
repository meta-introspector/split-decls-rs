macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! func_from_slice_variable_size {
    () => {
        deps!();
        # [cfg (feature = "safe_api")] # [doc = " Macro to implement a `from_slice()` function. Returns `UnknownCryptoError`"] # [doc = " if the slice is empty."] macro_rules ! func_from_slice_variable_size (($ name : ident) => (# [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [cfg (feature = "safe_api")] # [cfg_attr (docsrs , doc (cfg (feature = "safe_api")))] # [doc = " Construct from a given byte slice."] pub fn from_slice (slice : & [u8]) -> Result <$ name , UnknownCryptoError > { if slice . is_empty () || slice . len () > (isize :: MAX as usize) { return Err (UnknownCryptoError) ; } Ok ($ name { value : alloc :: vec :: Vec :: from (slice) , original_length : slice . len () }) })) ;
    };
}

func_from_slice_variable_size!();