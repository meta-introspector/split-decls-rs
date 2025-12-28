macro_rules! deps {
    () => {
        HANDLE!();
        ADDRESS64!();
    };
}

macro_rules! PTRANSLATE_ADDRESS_ROUTINE64 {
    () => {
        deps!();
        pub type PTRANSLATE_ADDRESS_ROUTINE64 = Option < unsafe extern "system" fn (hprocess : HANDLE , hthread : HANDLE , lpaddr : * const ADDRESS64) -> u64 , > ;
    };
}

PTRANSLATE_ADDRESS_ROUTINE64!()