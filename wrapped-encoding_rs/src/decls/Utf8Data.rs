macro_rules! Utf8Data {
    () => {
        # [repr (align (64))] pub struct Utf8Data { pub table : [u8 ; 384] , }
    };
}

Utf8Data!()