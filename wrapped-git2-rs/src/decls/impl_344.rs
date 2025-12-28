macro_rules! deps {
    () => {
        DiffLine!();
        DiffLineType!();
        Binding!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl < 'a > DiffLine < 'a > { # [doc = " Line number in old file or `None` for added line"] pub fn old_lineno (& self) -> Option < u32 > { match unsafe { (* self . raw) . old_lineno } { n if n < 0 => None , n => Some (n as u32) , } } # [doc = " Line number in new file or `None` for deleted line"] pub fn new_lineno (& self) -> Option < u32 > { match unsafe { (* self . raw) . new_lineno } { n if n < 0 => None , n => Some (n as u32) , } } # [doc = " Number of newline characters in content"] pub fn num_lines (& self) -> u32 { unsafe { (* self . raw) . num_lines as u32 } } # [doc = " Offset in the original file to the content"] pub fn content_offset (& self) -> i64 { unsafe { (* self . raw) . content_offset as i64 } } # [doc = " Content of this line as bytes."] pub fn content (& self) -> & 'a [u8] { unsafe { slice :: from_raw_parts ((* self . raw) . content as * const u8 , (* self . raw) . content_len as usize ,) } } # [doc = " origin of this `DiffLine`."] # [doc = ""] pub fn origin_value (& self) -> DiffLineType { unsafe { Binding :: from_raw ((* self . raw) . origin as raw :: git_diff_line_t) } } # [doc = " Sigil showing the origin of this `DiffLine`."] # [doc = ""] # [doc = "  * ` ` - Line context"] # [doc = "  * `+` - Line addition"] # [doc = "  * `-` - Line deletion"] # [doc = "  * `=` - Context (End of file)"] # [doc = "  * `>` - Add (End of file)"] # [doc = "  * `<` - Remove (End of file)"] # [doc = "  * `F` - File header"] # [doc = "  * `H` - Hunk header"] # [doc = "  * `B` - Line binary"] pub fn origin (& self) -> char { match unsafe { (* self . raw) . origin as raw :: git_diff_line_t } { raw :: GIT_DIFF_LINE_CONTEXT => ' ' , raw :: GIT_DIFF_LINE_ADDITION => '+' , raw :: GIT_DIFF_LINE_DELETION => '-' , raw :: GIT_DIFF_LINE_CONTEXT_EOFNL => '=' , raw :: GIT_DIFF_LINE_ADD_EOFNL => '>' , raw :: GIT_DIFF_LINE_DEL_EOFNL => '<' , raw :: GIT_DIFF_LINE_FILE_HDR => 'F' , raw :: GIT_DIFF_LINE_HUNK_HDR => 'H' , raw :: GIT_DIFF_LINE_BINARY => 'B' , _ => ' ' , } } }
    };
}

impl_344!();