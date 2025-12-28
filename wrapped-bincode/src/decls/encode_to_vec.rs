macro_rules! deps {
    () => {
        SerdeEncoder!();
        Encode!();
        EncodeError!();
        EncoderImpl!();
        VecWriter!();
        Config!();
    };
}

macro_rules! encode_to_vec {
    () => {
        deps!();
        # [doc = " Encode the given value into a `Vec<u8>` with the given `Config`. See the [config] module for more information."] # [doc = ""] # [doc = " [config]: ../config/index.html"] # [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub fn encode_to_vec < E , C > (val : E , config : C) -> Result < Vec < u8 > , EncodeError > where E : Serialize , C : Config , { let mut encoder = crate :: enc :: EncoderImpl :: new (crate :: VecWriter :: default () , config) ; let serializer = SerdeEncoder { enc : & mut encoder } ; val . serialize (serializer) ? ; Ok (encoder . into_writer () . collect ()) }
    };
}

encode_to_vec!();