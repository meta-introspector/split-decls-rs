macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! IndexObjectFn {
    () => {
        deps!();
        # [doc = " A function that writes a buffer like `fn(&mut buf)` with by tes of an object in the index that is the one that should be converted."] pub type IndexObjectFn < 'a > = dyn FnMut (& mut Vec < u8 >) -> Result < Option < () > , Box < dyn std :: error :: Error + Send + Sync > > + 'a ;
    };
}

IndexObjectFn!();