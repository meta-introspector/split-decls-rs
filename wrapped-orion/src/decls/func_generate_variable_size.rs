macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! func_generate_variable_size {
    () => {
        deps!();
        # [cfg (feature = "safe_api")] # [doc = " Macro to implement a `generate()` function for objects that benefit from"] # [doc = " having a CSPRNG available to generate data of a variable length."] macro_rules ! func_generate_variable_size (($ name : ident) => (# [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [cfg (feature = "safe_api")] # [cfg_attr (docsrs , doc (cfg (feature = "safe_api")))] # [doc = " Randomly generate using a CSPRNG. Not available in `no_std` context."] pub fn generate (length : usize) -> Result <$ name , UnknownCryptoError > { if length < 1 || length > (isize :: MAX as usize) { return Err (UnknownCryptoError) ; } let mut value = vec ! [0u8 ; length] ; crate :: util :: secure_rand_bytes (& mut value) . unwrap () ; Ok ($ name { value , original_length : length }) })) ;
    };
}

func_generate_variable_size!()