macro_rules! deps {
    () => {
        Pod!();
        Result!();
    };
}

macro_rules! BytesMut {
    () => {
        deps!();
        # [doc = " A trait for mutable byte slices."] # [doc = ""] # [doc = " It provides convenience methods for `Pod` types."] pub (crate) trait BytesMut { fn write_at < T : Pod > (self , offset : usize , val : & T) -> Result < () , () > ; }
    };
}

BytesMut!();