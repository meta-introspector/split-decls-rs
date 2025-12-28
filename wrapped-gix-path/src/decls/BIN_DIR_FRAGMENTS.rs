macro_rules! BIN_DIR_FRAGMENTS {
    () => {
        # [doc = " `bin` directory paths to try relative to the root of a Git for Windows or MSYS2 installation."] # [doc = ""] # [doc = " These are ordered so that a shim is preferred over a non-shim when they are tried in order."] const BIN_DIR_FRAGMENTS : & [& str] = & ["bin" , "usr/bin"] ;
    };
}

BIN_DIR_FRAGMENTS!();