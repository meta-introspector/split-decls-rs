macro_rules! deps {
    () => {
        ValueParserInner!();
        TypedValueParser!();
    };
}

macro_rules! ValueParser {
    () => {
        deps!();
        # [doc = " Parse/validate argument values"] # [doc = ""] # [doc = " Specified with [`Arg::value_parser`][crate::Arg::value_parser]."] # [doc = ""] # [doc = " `ValueParser` defines how to convert a raw argument value into a validated and typed value for"] # [doc = " use within an application."] # [doc = ""] # [doc = " See"] # [doc = " - [`value_parser!`][crate::value_parser] for automatically selecting an implementation for a given type"] # [doc = " - [`ValueParser::new`] for additional [`TypedValueParser`] that can be used"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " let mut cmd = clap::Command::new(\"raw\")"] # [doc = "     .arg("] # [doc = "         clap::Arg::new(\"color\")"] # [doc = "             .long(\"color\")"] # [doc = "             .value_parser([\"always\", \"auto\", \"never\"])"] # [doc = "             .default_value(\"auto\")"] # [doc = "     )"] # [doc = "     .arg("] # [doc = "         clap::Arg::new(\"hostname\")"] # [doc = "             .long(\"hostname\")"] # [doc = "             .value_parser(clap::builder::NonEmptyStringValueParser::new())"] # [doc = "             .action(clap::ArgAction::Set)"] # [doc = "             .required(true)"] # [doc = "     )"] # [doc = "     .arg("] # [doc = "         clap::Arg::new(\"port\")"] # [doc = "             .long(\"port\")"] # [doc = "             .value_parser(clap::value_parser!(u16).range(3000..))"] # [doc = "             .action(clap::ArgAction::Set)"] # [doc = "             .required(true)"] # [doc = "     );"] # [doc = ""] # [doc = " let m = cmd.try_get_matches_from_mut("] # [doc = "     [\"cmd\", \"--hostname\", \"rust-lang.org\", \"--port\", \"3001\"]"] # [doc = " ).unwrap();"] # [doc = ""] # [doc = " let color: &String = m.get_one(\"color\")"] # [doc = "     .expect(\"default\");"] # [doc = " assert_eq!(color, \"auto\");"] # [doc = ""] # [doc = " let hostname: &String = m.get_one(\"hostname\")"] # [doc = "     .expect(\"required\");"] # [doc = " assert_eq!(hostname, \"rust-lang.org\");"] # [doc = ""] # [doc = " let port: u16 = *m.get_one(\"port\")"] # [doc = "     .expect(\"required\");"] # [doc = " assert_eq!(port, 3001);"] # [doc = " ```"] pub struct ValueParser (ValueParserInner) ;
    };
}

ValueParser!()