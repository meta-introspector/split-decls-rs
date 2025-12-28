macro_rules! append_unquoted_key_value {
    () => {
        # [inline (never)] fn append_unquoted_key_value (out : & mut String , key : & str , value : & str) { out . push_str (key) ; out . push ('=') ; out . push_str (value) ; out . push_str (", ") ; }
    };
}

append_unquoted_key_value!();