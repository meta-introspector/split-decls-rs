macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! is_ansi_encoding {
    () => {
        deps!();
        fn is_ansi_encoding (row : Field) -> bool { row . find_attribute ("NativeEncodingAttribute") . is_some_and (| attribute | matches ! (attribute . args () . first () , Some ((_ , Value :: Str (encoding))) if * encoding == "ansi")) }
    };
}

is_ansi_encoding!()