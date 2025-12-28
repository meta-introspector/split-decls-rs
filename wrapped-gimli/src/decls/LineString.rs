macro_rules! LineString {
    () => {
        # [doc = " A string value for use in defining paths in line number programs."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum LineString { # [doc = " A slice of bytes representing a string. Must not include null bytes."] # [doc = " Not guaranteed to be UTF-8 or anything like that."] String (Vec < u8 >) , # [doc = " A reference to a string in the `.debug_str` section."] StringRef (StringId) , # [doc = " A reference to a string in the `.debug_line_str` section."] LineStringRef (LineStringId) , }
    };
}

LineString!()