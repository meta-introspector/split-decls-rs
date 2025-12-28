macro_rules! deps {
    () => {
        Decode!();
        Config!();
        DecodeError!();
    };
}

macro_rules! decode_from_slice {
    () => {
        deps!();
        # [doc = " Attempt to decode a given type `D` from the given slice. Returns the decoded output and the amount of bytes read."] # [doc = ""] # [doc = " Note that this does not work with borrowed types like `&str` or `&[u8]`. For that use [borrow_decode_from_slice]."] # [doc = ""] # [doc = " See the [config] module for more information on configurations."] # [doc = ""] # [doc = " [config]: config/index.html"] pub fn decode_from_slice < D : de :: Decode < () > , C : Config > (src : & [u8] , config : C ,) -> Result < (D , usize) , error :: DecodeError > { decode_from_slice_with_context (src , config , ()) }
    };
}

decode_from_slice!()