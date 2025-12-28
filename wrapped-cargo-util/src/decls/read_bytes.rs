macro_rules! read_bytes {
    () => {
        # [doc = " Reads a file into a bytes vector."] # [doc = ""] # [doc = " Equivalent to [`std::fs::read`] with better error messages."] pub fn read_bytes (path : & Path) -> Result < Vec < u8 > > { fs :: read (path) . with_context (| | format ! ("failed to read `{}`" , path . display ())) }
    };
}

read_bytes!();