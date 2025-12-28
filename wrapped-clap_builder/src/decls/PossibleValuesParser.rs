macro_rules! deps {
    () => {
        EnumValueParser!();
        Usage!();
        PossibleValue!();
    };
}

macro_rules! PossibleValuesParser {
    () => {
        deps!();
        # [doc = " Verify the value is from an enumerated set of [`PossibleValue`][crate::builder::PossibleValue]."] # [doc = ""] # [doc = " See also:"] # [doc = " - [`EnumValueParser`] for directly supporting [`ValueEnum`][crate::ValueEnum] types"] # [doc = " - [`TypedValueParser::map`] for adapting values to a more specialized type, like an external"] # [doc = "   enums that can't implement [`ValueEnum`][crate::ValueEnum]"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Usage:"] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " let mut cmd = clap::Command::new(\"raw\")"] # [doc = "     .arg("] # [doc = "         clap::Arg::new(\"color\")"] # [doc = "             .value_parser(clap::builder::PossibleValuesParser::new([\"always\", \"auto\", \"never\"]))"] # [doc = "             .required(true)"] # [doc = "     );"] # [doc = ""] # [doc = " let m = cmd.try_get_matches_from_mut([\"cmd\", \"always\"]).unwrap();"] # [doc = " let port: &String = m.get_one(\"color\")"] # [doc = "     .expect(\"required\");"] # [doc = " assert_eq!(port, \"always\");"] # [doc = " ```"] # [doc = ""] # [doc = " Semantics:"] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " # use std::ffi::OsStr;"] # [doc = " # use clap::builder::TypedValueParser;"] # [doc = " # let cmd = clap::Command::new(\"test\");"] # [doc = " # let arg = None;"] # [doc = " let value_parser = clap::builder::PossibleValuesParser::new([\"always\", \"auto\", \"never\"]);"] # [doc = " assert!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"random\")).is_err());"] # [doc = " assert!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"\")).is_err());"] # [doc = " assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"always\")).unwrap(), \"always\");"] # [doc = " assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"auto\")).unwrap(), \"auto\");"] # [doc = " assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"never\")).unwrap(), \"never\");"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct PossibleValuesParser (Vec < super :: PossibleValue >) ;
    };
}

PossibleValuesParser!();