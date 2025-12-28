macro_rules! deps {
    () => {
        OwnedSerdeDecoder!();
        IoReader!();
        Decode!();
        DecodeError!();
        Config!();
        DecoderImpl!();
    };
}

macro_rules! seed_decode_from_std_read {
    () => {
        deps!();
        # [doc = " Decode from the given reader with the given `Config` using a seed. The reader can be any type that implements `std::io::Read`, e.g. `std::fs::File`."] # [doc = ""] # [doc = " See the [config] module for more information about config options."] # [doc = ""] # [doc = " [config]: ../config/index.html"] # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub fn seed_decode_from_std_read < 'de , 'r , D , C , R > (seed : D , src : & 'r mut R , config : C ,) -> Result < D :: Value , DecodeError > where D : DeserializeSeed < 'de > , C : Config , R : std :: io :: Read , { let mut serde_decoder = OwnedSerdeDecoder :: < DecoderImpl < IoReader < & 'r mut R > , C , () > > :: from_std_read (src , config) ; seed . deserialize (serde_decoder . as_deserializer ()) }
    };
}

seed_decode_from_std_read!()