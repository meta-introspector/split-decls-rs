macro_rules! deps {
    () => {
        Decompress!();
    };
}

macro_rules! Inflate {
    () => {
        deps!();
        # [doc = " Decompress a few bytes of a zlib stream without allocation"] # [derive (Default)] pub struct Inflate { # [doc = " The actual decompressor doing all the work."] pub state : Decompress , }
    };
}

Inflate!()