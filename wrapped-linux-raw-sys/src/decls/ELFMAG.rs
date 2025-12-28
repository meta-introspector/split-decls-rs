macro_rules! ELFMAG {
    () => {
        pub const ELFMAG : [u8 ; SELFMAG] = [0x7f , b'E' , b'L' , b'F'] ;
    };
}

ELFMAG!()