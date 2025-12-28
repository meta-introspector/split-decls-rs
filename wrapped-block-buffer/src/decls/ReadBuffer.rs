macro_rules! ReadBuffer {
    () => {
        # [doc = " Buffer for reading block-generated data."] pub struct ReadBuffer < BS : ArraySize > { # [doc = " The first byte of the block is used as cursor position."] # [doc = " `&buffer[usize::from(buffer[0])..]` is interpreted as unread bytes."] # [doc = " The cursor position is always bigger than zero and smaller than or equal to block size."] buffer : Array < u8 , BS > , }
    };
}

ReadBuffer!()