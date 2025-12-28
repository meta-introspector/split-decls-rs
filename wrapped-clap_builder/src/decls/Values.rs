macro_rules! deps {
    () => {
        ArgAction!();
        Arg!();
        AnyValue!();
    };
}

macro_rules! Values {
    () => {
        deps!();
        # [doc = " Iterate over multiple values for an argument via [`ArgMatches::remove_many`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " # use clap::{Command, Arg, ArgAction};"] # [doc = " let mut m = Command::new(\"myapp\")"] # [doc = "     .arg(Arg::new(\"output\")"] # [doc = "         .short('o')"] # [doc = "         .action(ArgAction::Append))"] # [doc = "     .get_matches_from(vec![\"myapp\", \"-o\", \"val1\", \"-o\", \"val2\"]);"] # [doc = ""] # [doc = " let mut values = m.remove_many::<String>(\"output\")"] # [doc = "     .unwrap();"] # [doc = ""] # [doc = " assert_eq!(values.next(), Some(String::from(\"val1\")));"] # [doc = " assert_eq!(values.next(), Some(String::from(\"val2\")));"] # [doc = " assert_eq!(values.next(), None);"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct Values < T > { # [allow (clippy :: type_complexity)] iter : Map < Flatten < std :: vec :: IntoIter < Vec < AnyValue > > > , fn (AnyValue) -> T > , len : usize , }
    };
}

Values!()