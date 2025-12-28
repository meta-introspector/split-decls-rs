macro_rules! Byte {
    () => {
        # [doc = " A type that wraps a single byte with a convenient fmt::Debug impl that"] # [doc = " escapes the byte."] pub (crate) struct Byte (pub (crate) u8) ;
    };
}

Byte!()