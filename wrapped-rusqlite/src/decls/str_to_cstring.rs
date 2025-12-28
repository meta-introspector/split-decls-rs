macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! str_to_cstring {
    () => {
        deps!();
        # [cfg (any (feature = "functions" , feature = "vtab" , test))] fn str_to_cstring (s : & str) -> Result < util :: SmallCString > { Ok (util :: SmallCString :: new (s) ?) }
    };
}

str_to_cstring!()