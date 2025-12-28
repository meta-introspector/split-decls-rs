macro_rules! deps {
    () => {
        Usage!();
        BoolishValueParser!();
    };
}

macro_rules! FalseyValueParser {
    () => {
        deps!();
        # [doc = " Parse false-like string values, everything else is `true`"] # [doc = ""] # [doc = " See also:"] # [doc = " - [`ValueParser::bool`] for assuming non-false is true"] # [doc = " - [`BoolishValueParser`] for different human readable bool representations"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Usage:"] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " let mut cmd = clap::Command::new(\"raw\")"] # [doc = "     .arg("] # [doc = "         clap::Arg::new(\"append\")"] # [doc = "             .value_parser(clap::builder::FalseyValueParser::new())"] # [doc = "             .required(true)"] # [doc = "     );"] # [doc = ""] # [doc = " let m = cmd.try_get_matches_from_mut([\"cmd\", \"true\"]).unwrap();"] # [doc = " let port: bool = *m.get_one(\"append\")"] # [doc = "     .expect(\"required\");"] # [doc = " assert_eq!(port, true);"] # [doc = " ```"] # [doc = ""] # [doc = " Semantics:"] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " # use std::ffi::OsStr;"] # [doc = " # use clap::builder::TypedValueParser;"] # [doc = " # let cmd = clap::Command::new(\"test\");"] # [doc = " # let arg = None;"] # [doc = " let value_parser = clap::builder::FalseyValueParser::new();"] # [doc = " assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"random\")).unwrap(), true);"] # [doc = " assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"100\")).unwrap(), true);"] # [doc = " assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"\")).unwrap(), false);"] # [doc = " assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"false\")).unwrap(), false);"] # [doc = " assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"No\")).unwrap(), false);"] # [doc = " assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"oFF\")).unwrap(), false);"] # [doc = " assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"0\")).unwrap(), false);"] # [doc = " ```"] # [derive (Copy , Clone , Debug)] # [non_exhaustive] pub struct FalseyValueParser { }
    };
}

FalseyValueParser!();