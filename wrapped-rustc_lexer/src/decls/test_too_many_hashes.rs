macro_rules! deps {
    () => {
        RawStrError!();
    };
}

macro_rules! test_too_many_hashes {
    () => {
        deps!();
        # [test] fn test_too_many_hashes () { let max_count = u8 :: MAX ; let hashes1 = "#" . repeat (max_count as usize) ; let hashes2 = "#" . repeat (max_count as usize + 1) ; let middle = "\"abc\"" ; let s1 = [& hashes1 , middle , & hashes1] . join ("") ; let s2 = [& hashes2 , middle , & hashes2] . join ("") ; check_raw_str (& s1 , Ok (255)) ; check_raw_str (& s2 , Err (RawStrError :: TooManyDelimiters { found : u32 :: from (max_count) + 1 })) ; }
    };
}

test_too_many_hashes!()