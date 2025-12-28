macro_rules! is_fast {
    () => {
        # [doc = " Whether RK is believed to be very fast for the given needle/haystack."] # [inline] pub (crate) fn is_fast (haystack : & [u8] , _needle : & [u8]) -> bool { haystack . len () < 16 }
    };
}

is_fast!();