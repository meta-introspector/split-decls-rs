macro_rules! func_generate {
    () => {
        # [doc = " Macro to implement a `generate()` function for objects that benefit from"] # [doc = " having a CSPRNG available to generate data of a fixed length $gen_length."] macro_rules ! func_generate (($ name : ident , $ upper_bound : expr , $ gen_length : expr) => (# [cfg (feature = "safe_api")] # [cfg_attr (docsrs , doc (cfg (feature = "safe_api")))] # [doc = " Randomly generate using a CSPRNG. Not available in `no_std` context."] pub fn generate () -> $ name { let mut value = [0u8 ; $ upper_bound] ; crate :: util :: secure_rand_bytes (& mut value [..$ gen_length]) . unwrap () ; $ name { value , original_length : $ gen_length } })) ;
    };
}

func_generate!()