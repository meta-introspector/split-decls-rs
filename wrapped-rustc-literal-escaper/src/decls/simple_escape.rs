macro_rules! simple_escape {
    () => {
        # [doc = " Interpret a non-nul ASCII escape"] # [doc = ""] # [doc = " Parses the character of an ASCII escape (except nul) without the leading backslash."] # [inline] fn simple_escape (c : char) -> Result < NonZero < u8 > , char > { Ok (NonZero :: new (match c { '"' => b'"' , 'n' => b'\n' , 'r' => b'\r' , 't' => b'\t' , '\\' => b'\\' , '\'' => b'\'' , _ => Err (c) ? , }) . unwrap ()) }
    };
}

simple_escape!()