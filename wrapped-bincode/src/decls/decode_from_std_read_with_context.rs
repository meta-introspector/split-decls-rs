macro_rules! deps {
    () => {
        IoReader!();
        DecoderImpl!();
        DecodeError!();
        Decode!();
        Config!();
    };
}

macro_rules! decode_from_std_read_with_context {
    () => {
        deps!();
        # [doc = " Decode type `D` from the given reader with the given `Config` and `Context`. The reader can be any type that implements `std::io::Read`, e.g. `std::fs::File`."] # [doc = ""] # [doc = " See the [config] module for more information about config options."] # [doc = ""] # [doc = " [config]: config/index.html"] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub fn decode_from_std_read_with_context < Context , D : Decode < Context > , C : Config , R : std :: io :: Read , > (src : & mut R , config : C , context : Context ,) -> Result < D , DecodeError > { let reader = IoReader :: new (src) ; let mut decoder = DecoderImpl :: < _ , C , Context > :: new (reader , config , context) ; D :: decode (& mut decoder) }
    };
}

decode_from_std_read_with_context!()