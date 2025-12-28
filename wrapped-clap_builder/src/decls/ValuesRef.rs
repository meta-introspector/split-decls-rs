macro_rules! deps {
    () => {
        Arg!();
        ArgAction!();
        Iter!();
        AnyValue!();
    };
}

macro_rules! ValuesRef {
    () => {
        deps!();
        # [doc = " Iterate over multiple values for an argument via [`ArgMatches::get_many`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " # use clap::{Command, Arg, ArgAction};"] # [doc = " let m = Command::new(\"myapp\")"] # [doc = "     .arg(Arg::new(\"output\")"] # [doc = "         .short('o')"] # [doc = "         .action(ArgAction::Append))"] # [doc = "     .get_matches_from(vec![\"myapp\", \"-o\", \"val1\", \"-o\", \"val2\"]);"] # [doc = ""] # [doc = " let mut values = m.get_many::<String>(\"output\")"] # [doc = "     .unwrap()"] # [doc = "     .map(|s| s.as_str());"] # [doc = ""] # [doc = " assert_eq!(values.next(), Some(\"val1\"));"] # [doc = " assert_eq!(values.next(), Some(\"val2\"));"] # [doc = " assert_eq!(values.next(), None);"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct ValuesRef < 'a , T > { # [allow (clippy :: type_complexity)] iter : Map < Flatten < Iter < 'a , Vec < AnyValue > > > , fn (& AnyValue) -> & T > , len : usize , }
    };
}

ValuesRef!();