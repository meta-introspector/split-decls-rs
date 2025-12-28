macro_rules! deps {
    () => {
        Line!();
    };
}

macro_rules! LineEnding {
    () => {
        deps!();
        # [doc = " Line endings: variants of newline characters that can be used with Base64."] # [doc = ""] # [doc = " Use [`LineEnding::default`] to get an appropriate line ending for the"] # [doc = " current operating system."] # [allow (clippy :: upper_case_acronyms)] # [derive (Copy , Clone , Debug , Eq , PartialEq , PartialOrd , Ord)] pub enum LineEnding { # [doc = " Carriage return: `\\r` (Pre-OS X Macintosh)"] CR , # [doc = " Line feed: `\\n` (Unix OSes)"] LF , # [doc = " Carriage return + line feed: `\\r\\n` (Windows)"] CRLF , }
    };
}

LineEnding!();