macro_rules! deps {
    () => {
        OwnedSerdeDecoder!();
        Config!();
        DecodeError!();
        Decode!();
        DecoderImpl!();
        IoReader!();
    };
}

macro_rules! decode_from_std_read {
    () => {
        deps!();
        # [doc = " Decode type `D` from the given reader with the given `Config`. The reader can be any type that implements `std::io::Read`, e.g. `std::fs::File`."] # [doc = ""] # [doc = " See the [config] module for more information about config options."] # [doc = ""] # [doc = " [config]: ../config/index.html"] # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub fn decode_from_std_read < 'r , D : DeserializeOwned , C : Config , R : std :: io :: Read > (src : & 'r mut R , config : C ,) -> Result < D , DecodeError > { let mut serde_decoder = OwnedSerdeDecoder :: < DecoderImpl < IoReader < & 'r mut R > , C , () > > :: from_std_read (src , config) ; D :: deserialize (serde_decoder . as_deserializer ()) }
    };
}

decode_from_std_read!()