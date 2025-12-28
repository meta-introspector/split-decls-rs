macro_rules! deps {
    () => {
        IoWriter!();
        Config!();
        EncoderImpl!();
        EncodeError!();
        SerdeEncoder!();
        Encode!();
    };
}

macro_rules! encode_into_std_write {
    () => {
        deps!();
        # [doc = " Encode the given value into any type that implements `std::io::Write`, e.g. `std::fs::File`, with the given `Config`."] # [doc = " See the [config] module for more information."] # [doc = ""] # [doc = " [config]: ../config/index.html"] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] # [cfg (feature = "std")] pub fn encode_into_std_write < E : Serialize , C : Config , W : std :: io :: Write > (val : E , dst : & mut W , config : C ,) -> Result < usize , EncodeError > { let writer = crate :: IoWriter :: new (dst) ; let mut encoder = crate :: enc :: EncoderImpl :: < _ , C > :: new (writer , config) ; let serializer = SerdeEncoder { enc : & mut encoder } ; val . serialize (serializer) ? ; Ok (encoder . into_writer () . bytes_written ()) }
    };
}

encode_into_std_write!()