macro_rules! AsBytes {
    () => {
        # [doc = " Helper trait for types that can be viewed as a byte slice"] pub trait AsBytes { # [doc = " Casts the input type to a byte slice"] fn as_bytes (& self) -> & [u8] ; }
    };
}

AsBytes!();