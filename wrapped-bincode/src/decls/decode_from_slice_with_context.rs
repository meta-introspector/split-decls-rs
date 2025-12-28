macro_rules! deps {
    () => {
        SliceReader!();
        Config!();
        DecoderImpl!();
        Decode!();
        DecodeError!();
    };
}

macro_rules! decode_from_slice_with_context {
    () => {
        deps!();
        # [doc = " Attempt to decode a given type `D` from the given slice with `Context`. Returns the decoded output and the amount of bytes read."] # [doc = ""] # [doc = " Note that this does not work with borrowed types like `&str` or `&[u8]`. For that use [borrow_decode_from_slice]."] # [doc = ""] # [doc = " See the [config] module for more information on configurations."] # [doc = ""] # [doc = " [config]: config/index.html"] pub fn decode_from_slice_with_context < Context , D : de :: Decode < Context > , C : Config > (src : & [u8] , config : C , context : Context ,) -> Result < (D , usize) , error :: DecodeError > { let reader = de :: read :: SliceReader :: new (src) ; let mut decoder = de :: DecoderImpl :: < _ , C , Context > :: new (reader , config , context) ; let result = D :: decode (& mut decoder) ? ; let bytes_read = src . len () - decoder . reader () . slice . len () ; Ok ((result , bytes_read)) }
    };
}

decode_from_slice_with_context!();