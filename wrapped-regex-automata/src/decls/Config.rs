macro_rules! Config {
    () => {
        # [doc = " A common set of configuration options that apply to the syntax of a regex."] # [doc = ""] # [doc = " This represents a group of configuration options that specifically apply"] # [doc = " to how the concrete syntax of a regular expression is interpreted. In"] # [doc = " particular, they are generally forwarded to the"] # [doc = " [`ParserBuilder`](https://docs.rs/regex-syntax/*/regex_syntax/struct.ParserBuilder.html)"] # [doc = " in the"] # [doc = " [`regex-syntax`](https://docs.rs/regex-syntax)"] # [doc = " crate when building a regex from its concrete syntax directly."] # [doc = ""] # [doc = " These options are defined as a group since they apply to every regex engine"] # [doc = " in this crate. Instead of re-defining them on every engine's builder, they"] # [doc = " are instead provided here as one cohesive unit."] # [derive (Clone , Copy , Debug)] pub struct Config { case_insensitive : bool , multi_line : bool , dot_matches_new_line : bool , crlf : bool , line_terminator : u8 , swap_greed : bool , ignore_whitespace : bool , unicode : bool , utf8 : bool , nest_limit : u32 , octal : bool , }
    };
}

Config!()