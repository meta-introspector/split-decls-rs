macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! get_dict_size {
    () => {
        deps!();
        fn get_dict_size (dict_size : u32) -> crate :: Result < u32 > { if dict_size > DICT_SIZE_MAX { return Err (error_invalid_input ("dict size too large")) ; } let dict_size = dict_size . max (4096) ; Ok ((dict_size + 15) & ! 15) }
    };
}

get_dict_size!()