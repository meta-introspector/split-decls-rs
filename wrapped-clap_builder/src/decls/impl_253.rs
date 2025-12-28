macro_rules! deps {
    () => {
        ValueParser!();
        TypedValueParser!();
    };
}

macro_rules! impl_253 {
    () => {
        deps!();
        # [doc = " Convert a [`TypedValueParser`] to [`ValueParser`]"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " let mut cmd = clap::Command::new(\"raw\")"] # [doc = "     .arg("] # [doc = "         clap::Arg::new(\"hostname\")"] # [doc = "             .long(\"hostname\")"] # [doc = "             .value_parser(clap::builder::NonEmptyStringValueParser::new())"] # [doc = "             .action(clap::ArgAction::Set)"] # [doc = "             .required(true)"] # [doc = "     );"] # [doc = ""] # [doc = " let m = cmd.try_get_matches_from_mut("] # [doc = "     [\"cmd\", \"--hostname\", \"rust-lang.org\"]"] # [doc = " ).unwrap();"] # [doc = ""] # [doc = " let hostname: &String = m.get_one(\"hostname\")"] # [doc = "     .expect(\"required\");"] # [doc = " assert_eq!(hostname, \"rust-lang.org\");"] # [doc = " ```"] impl < P > From < P > for ValueParser where P : TypedValueParser + Send + Sync + 'static , { fn from (p : P) -> Self { Self :: new (p) } }
    };
}

impl_253!();