macro_rules! LzipHeader {
    () => {
        # [derive (Debug , Clone)] pub (crate) struct LzipHeader { version : u8 , dict_size : u32 , }
    };
}

LzipHeader!();