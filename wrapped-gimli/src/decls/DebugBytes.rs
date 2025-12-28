macro_rules! DebugBytes {
    () => {
        struct DebugBytes < 'input > (& 'input [u8]) ;
    };
}

DebugBytes!();