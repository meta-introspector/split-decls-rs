macro_rules! MacroDylib {
    () => {
        # [doc = " Represents a dynamically loaded library containing procedural macros."] pub struct MacroDylib { path : AbsPathBuf , }
    };
}

MacroDylib!();