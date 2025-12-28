macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! to_vec {
    () => {
        deps!();
        # [doc = " Serialize the given data structure as a JSON byte vector."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Serialization can fail if `T`'s implementation of `Serialize` decides to"] # [doc = " fail, or if `T` contains a map with non-string keys."] # [inline] pub fn to_vec < T > (value : & T) -> Result < Vec < u8 > > where T : ? Sized + Serialize , { let mut writer = Vec :: with_capacity (128) ; tri ! (to_writer (& mut writer , value)) ; Ok (writer) }
    };
}

to_vec!()