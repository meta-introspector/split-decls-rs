macro_rules! deps {
    () => {
        Usage!();
    };
}

macro_rules! RangedI64ValueParser {
    () => {
        deps!();
        # [doc = " Parse number that fall within a range of values"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **NOTE:** To capture negative values, you will also need to set"] # [doc = " [`Arg::allow_negative_numbers`][crate::Arg::allow_negative_numbers] or"] # [doc = " [`Arg::allow_hyphen_values`][crate::Arg::allow_hyphen_values]."] # [doc = ""] # [doc = " </div>"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Usage:"] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " let mut cmd = clap::Command::new(\"raw\")"] # [doc = "     .arg("] # [doc = "         clap::Arg::new(\"port\")"] # [doc = "             .long(\"port\")"] # [doc = "             .value_parser(clap::value_parser!(u16).range(3000..))"] # [doc = "             .action(clap::ArgAction::Set)"] # [doc = "             .required(true)"] # [doc = "     );"] # [doc = ""] # [doc = " let m = cmd.try_get_matches_from_mut([\"cmd\", \"--port\", \"3001\"]).unwrap();"] # [doc = " let port: u16 = *m.get_one(\"port\")"] # [doc = "     .expect(\"required\");"] # [doc = " assert_eq!(port, 3001);"] # [doc = " ```"] # [doc = ""] # [doc = " Semantics:"] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " # use std::ffi::OsStr;"] # [doc = " # use clap::builder::TypedValueParser;"] # [doc = " # let cmd = clap::Command::new(\"test\");"] # [doc = " # let arg = None;"] # [doc = " let value_parser = clap::builder::RangedI64ValueParser::<i32>::new().range(-1..200);"] # [doc = " assert!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"random\")).is_err());"] # [doc = " assert!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"\")).is_err());"] # [doc = " assert!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"-200\")).is_err());"] # [doc = " assert!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"300\")).is_err());"] # [doc = " assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"-1\")).unwrap(), -1);"] # [doc = " assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"0\")).unwrap(), 0);"] # [doc = " assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new(\"50\")).unwrap(), 50);"] # [doc = " ```"] # [derive (Copy , Clone , Debug)] pub struct RangedI64ValueParser < T : TryFrom < i64 > + Clone + Send + Sync = i64 > { bounds : (std :: ops :: Bound < i64 > , std :: ops :: Bound < i64 >) , target : std :: marker :: PhantomData < T > , }
    };
}

RangedI64ValueParser!();