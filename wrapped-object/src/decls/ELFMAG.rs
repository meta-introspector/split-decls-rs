macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! ELFMAG {
    () => {
        deps!();
        # [doc = " File identification bytes stored in `Ident::magic`."] pub const ELFMAG : [u8 ; 4] = [0x7f , b'E' , b'L' , b'F'] ;
    };
}

ELFMAG!()