macro_rules! XZ_MAGIC {
    () => {
        const XZ_MAGIC : [u8 ; 6] = [0xFD , b'7' , b'z' , b'X' , b'Z' , 0x00] ;
    };
}

XZ_MAGIC!();