macro_rules! DebugLoc {
    () => {
        # [doc = " A source code location used to generate debug information."] struct DebugLoc { # [doc = " Information about the original source file."] file : Arc < SourceFile > , # [doc = " The (1-based) line number."] line : u32 , # [doc = " The (1-based) column number."] col : u32 , }
    };
}

DebugLoc!();