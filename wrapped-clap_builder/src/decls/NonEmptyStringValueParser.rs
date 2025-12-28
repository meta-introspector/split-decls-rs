macro_rules! deps {
    () => {
        Usage!();
    };
}

macro_rules! NonEmptyStringValueParser {
    () => {
        deps!();
        # [doc = " Parse non-empty string values"] # [doc = ""] # [doc = " See also:"] # [doc = " - [`ValueParser::string`]"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Usage:"] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " let mut cmd = clap::Command::new(\"raw\")"] # [doc = "     .arg("] # [doc = "         clap::Arg::new(\"append\")"] # [doc = "             .value_parser(clap::builder::NonEmptyStringValueParser::new())"] # [doc = "             .required(true)"] # [doc = "     );"] # [doc = ""] # [doc = " let m = cmd.try_get_matches_from_mut([\"cmd\", \"true\"]).unwrap();"] # [doc = " let port: &String = m.get_one(\"append\")"] # [doc = "     .expect(\"required\");"] # [doc = " assert_eq!(port, \"true\");"] # [doc = " ```"] # [doc = ""] # [doc = " Semantics:"] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " # use std::ffi::OsStr;"] # [doc = " # use clap::builder::TypedValueParser;"] # [doc = " # let cmd = clap::Command::new(\"test\");"] # [doc = " # let arg = None;"] # [doc = " let value_parser = clap::builder::NonEmptyStringValueParser::new();"] # [doc = " assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"random\")).unwrap(), \"random\");"] # [doc = " assert!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"\")).is_err());"] # [doc = " ```"] # [derive (Copy , Clone , Debug)] # [non_exhaustive] pub struct NonEmptyStringValueParser { }
    };
}

NonEmptyStringValueParser!();