macro_rules! deps {
    () => {
        Result!();
        ErrorCode!();
        Error!();
        Read!();
        Deserializer!();
        StreamDeserializer!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < 'de , R , T > StreamDeserializer < 'de , R , T > where R : read :: Read < 'de > , T : de :: Deserialize < 'de > , { # [doc = " Create a JSON stream deserializer from one of the possible serde_json"] # [doc = " input sources."] # [doc = ""] # [doc = " Typically it is more convenient to use one of these methods instead:"] # [doc = ""] # [doc = "   - Deserializer::from_str(...).into_iter()"] # [doc = "   - Deserializer::from_slice(...).into_iter()"] # [doc = "   - Deserializer::from_reader(...).into_iter()"] pub fn new (read : R) -> Self { let offset = read . byte_offset () ; StreamDeserializer { de : Deserializer :: new (read) , offset , failed : false , output : PhantomData , lifetime : PhantomData , } } # [doc = " Returns the number of bytes so far deserialized into a successful `T`."] # [doc = ""] # [doc = " If a stream deserializer returns an EOF error, new data can be joined to"] # [doc = " `old_data[stream.byte_offset()..]` to try again."] # [doc = ""] # [doc = " ```"] # [doc = " let data = b\"[0] [1] [\";"] # [doc = ""] # [doc = " let de = serde_json::Deserializer::from_slice(data);"] # [doc = " let mut stream = de.into_iter::<Vec<i32>>();"] # [doc = " assert_eq!(0, stream.byte_offset());"] # [doc = ""] # [doc = " println!(\"{:?}\", stream.next()); // [0]"] # [doc = " assert_eq!(3, stream.byte_offset());"] # [doc = ""] # [doc = " println!(\"{:?}\", stream.next()); // [1]"] # [doc = " assert_eq!(7, stream.byte_offset());"] # [doc = ""] # [doc = " println!(\"{:?}\", stream.next()); // error"] # [doc = " assert_eq!(8, stream.byte_offset());"] # [doc = ""] # [doc = " // If err.is_eof(), can join the remaining data to new data and continue."] # [doc = " let remaining = &data[stream.byte_offset()..];"] # [doc = " ```"] # [doc = ""] # [doc = " *Note:* In the future this method may be changed to return the number of"] # [doc = " bytes so far deserialized into a successful T *or* syntactically valid"] # [doc = " JSON skipped over due to a type error. See [serde-rs/json#70] for an"] # [doc = " example illustrating this."] # [doc = ""] # [doc = " [serde-rs/json#70]: https://github.com/serde-rs/json/issues/70"] pub fn byte_offset (& self) -> usize { self . offset } fn peek_end_of_value (& mut self) -> Result < () > { match tri ! (self . de . peek ()) { Some (b' ' | b'\n' | b'\t' | b'\r' | b'"' | b'[' | b']' | b'{' | b'}' | b',' | b':') | None => Ok (()) , Some (_) => { let position = self . de . read . peek_position () ; Err (Error :: syntax (ErrorCode :: TrailingCharacters , position . line , position . column ,)) } } } }
    };
}

impl_47!();