macro_rules! deps {
    () => {
        DebugLineStrOffset!();
        Result!();
        Reader!();
        DebugLineStr!();
    };
}

macro_rules! impl_596 {
    () => {
        deps!();
        impl < R : Reader > DebugLineStr < R > { # [doc = " Lookup a string from the `.debug_line_str` section by DebugLineStrOffset."] pub fn get_str (& self , offset : DebugLineStrOffset < R :: Offset >) -> Result < R > { let input = & mut self . section . clone () ; input . skip (offset . 0) ? ; input . read_null_terminated_slice () } }
    };
}

impl_596!();