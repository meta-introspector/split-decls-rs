macro_rules! Bytes {
    () => {
        # [doc = " Parallel iterator over the bytes of a string"] # [derive (Debug , Clone)] pub struct Bytes < 'ch > { chars : & 'ch str , }
    };
}

Bytes!();