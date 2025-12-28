macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! STT_TLS {
    () => {
        deps!();
        # [doc = " Symbol is a thread-local storage object."] pub const STT_TLS : u8 = 6 ;
    };
}

STT_TLS!()