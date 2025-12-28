macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! raw_name_value {
    () => {
        deps!();
        unsafe fn raw_name_value (name : * const u8 , name_len : size_t , value : * const u8 , value_len : size_t ,) -> Result < (HeaderName , HeaderValue , Bytes) , hyper_code > { let name = std :: slice :: from_raw_parts (name , name_len) ; let orig_name = Bytes :: copy_from_slice (name) ; let name = match HeaderName :: from_bytes (name) { Ok (name) => name , Err (_) => return Err (hyper_code :: HYPERE_INVALID_ARG) , } ; let value = std :: slice :: from_raw_parts (value , value_len) ; let value = match HeaderValue :: from_bytes (value) { Ok (val) => val , Err (_) => return Err (hyper_code :: HYPERE_INVALID_ARG) , } ; Ok ((name , value , orig_name)) }
    };
}

raw_name_value!()