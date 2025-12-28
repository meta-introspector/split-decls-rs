macro_rules! deps {
    () => {
        Deserializer!();
        Read!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < 'de , R > Deserializer < R > where R : read :: Read < 'de > , { # [doc = " Create a JSON deserializer from one of the possible serde_json input"] # [doc = " sources."] # [doc = ""] # [doc = " When reading from a source against which short reads are not efficient, such"] # [doc = " as a [`File`], you will want to apply your own buffering because serde_json"] # [doc = " will not buffer the input. See [`std::io::BufReader`]."] # [doc = ""] # [doc = " Typically it is more convenient to use one of these methods instead:"] # [doc = ""] # [doc = "   - Deserializer::from_str"] # [doc = "   - Deserializer::from_slice"] # [doc = "   - Deserializer::from_reader"] # [doc = ""] # [doc = " [`File`]: std::fs::File"] pub fn new (read : R) -> Self { Deserializer { read , scratch : Vec :: new () , remaining_depth : 128 , # [cfg (feature = "float_roundtrip")] single_precision : false , # [cfg (feature = "unbounded_depth")] disable_recursion_limit : false , } } }
    };
}

impl_13!();