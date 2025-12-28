macro_rules! WOKEN {
    () => {
        # [doc = " The stream was waked and will be polled."] const WOKEN : u8 = 0b10000 ;
    };
}

WOKEN!();