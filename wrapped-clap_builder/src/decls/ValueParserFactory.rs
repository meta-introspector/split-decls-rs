macro_rules! deps {
    () => {
        ValueParser!();
        Parser!();
    };
}

macro_rules! ValueParserFactory {
    () => {
        deps!();
        # [doc = " Register a type with [`value_parser!`][crate::value_parser!]"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " #[derive(Copy, Clone, Debug)]"] # [doc = " pub struct Custom(u32);"] # [doc = ""] # [doc = " impl clap::builder::ValueParserFactory for Custom {"] # [doc = "     type Parser = CustomValueParser;"] # [doc = "     fn value_parser() -> Self::Parser {"] # [doc = "         CustomValueParser"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " #[derive(Clone, Debug)]"] # [doc = " pub struct CustomValueParser;"] # [doc = " impl clap::builder::TypedValueParser for CustomValueParser {"] # [doc = "     type Value = Custom;"] # [doc = ""] # [doc = "     fn parse_ref("] # [doc = "         &self,"] # [doc = "         cmd: &clap::Command,"] # [doc = "         arg: Option<&clap::Arg>,"] # [doc = "         value: &std::ffi::OsStr,"] # [doc = "     ) -> Result<Self::Value, clap::Error> {"] # [doc = "         let inner = clap::value_parser!(u32);"] # [doc = "         let val = inner.parse_ref(cmd, arg, value)?;"] # [doc = "         Ok(Custom(val))"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let parser: CustomValueParser = clap::value_parser!(Custom);"] # [doc = " ```"] pub trait ValueParserFactory { # [doc = " Generated parser, usually [`ValueParser`]."] # [doc = ""] # [doc = " It should at least be a type that supports `Into<ValueParser>`.  A non-`ValueParser` type"] # [doc = " allows the caller to do further initialization on the parser."] type Parser ; # [doc = " Create the specified [`Self::Parser`]"] fn value_parser () -> Self :: Parser ; }
    };
}

ValueParserFactory!();