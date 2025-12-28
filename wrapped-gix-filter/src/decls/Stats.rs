macro_rules! Stats {
    () => {
        # [doc = " Statistics about a buffer that helps to safely perform EOL conversions"] # [derive (Debug , Default , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct Stats { # [doc = " The amount of null bytes."] pub null : usize , # [doc = " The amount of lone carriage returns (`\\r`)."] pub lone_cr : usize , # [doc = " The amount of lone line feeds (`\\n`)."] pub lone_lf : usize , # [doc = " The amount carriage returns followed by line feeds"] pub crlf : usize , # [doc = " The estimate of printable characters."] pub printable : usize , # [doc = " The estimate of characters that can't be printed."] pub non_printable : usize , }
    };
}

Stats!();