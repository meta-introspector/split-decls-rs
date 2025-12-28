macro_rules! read_vari32 {
    () => {
        # [doc = " Read a signed 32-bit integer using zig-zag encoding. Also, return the"] # [doc = " number of bytes read."] # [doc = ""] # [doc = " https://developers.google.com/protocol-buffers/docs/encoding#varints"] fn read_vari32 (data : & [u8]) -> (i32 , usize) { let (un , i) = read_varu32 (data) ; let mut n = i32 :: from_bits (un >> 1) ; if un & 1 != 0 { n = ! n ; } (n , i) }
    };
}

read_vari32!()