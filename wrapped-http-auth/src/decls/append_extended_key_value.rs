macro_rules! append_extended_key_value {
    () => {
        fn append_extended_key_value (out : & mut String , key : & str , value : & str) { out . push_str (key) ; out . push_str ("*=UTF-8''") ; for & b in value . as_bytes () { if (char_classes (b) & C_ATTR) != 0 { out . push (char :: from (b)) ; } else { let _ = write ! (out , "%{:02X}" , b) ; } } out . push_str (", ") ; }
    };
}

append_extended_key_value!()