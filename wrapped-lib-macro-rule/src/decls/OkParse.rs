macro_rules! OkParse {
    () => {
        # [doc = " Success variant."] pub struct OkParse < T > (pub T) ;
    };
}

OkParse!();