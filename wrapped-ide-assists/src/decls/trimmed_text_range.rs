macro_rules! trimmed_text_range {
    () => {
        # [doc = " Trim(remove leading and trailing whitespace) `initial_range` in `source_file`, return the trimmed range."] pub (crate) fn trimmed_text_range (source_file : & SourceFile , initial_range : TextRange) -> TextRange { let mut trimmed_range = initial_range ; while source_file . syntax () . token_at_offset (trimmed_range . start ()) . find_map (Whitespace :: cast) . is_some () && trimmed_range . start () < trimmed_range . end () { let start = trimmed_range . start () + TextSize :: from (1) ; trimmed_range = TextRange :: new (start , trimmed_range . end ()) ; } while source_file . syntax () . token_at_offset (trimmed_range . end ()) . find_map (Whitespace :: cast) . is_some () && trimmed_range . start () < trimmed_range . end () { let end = trimmed_range . end () - TextSize :: from (1) ; trimmed_range = TextRange :: new (trimmed_range . start () , end) ; } trimmed_range }
    };
}

trimmed_text_range!()