macro_rules! deps {
    () => {
        Id!();
        Iter!();
    };
}

macro_rules! IdsRef {
    () => {
        deps!();
        # [doc = " Iterate over [`Arg`][crate::Arg] and [`ArgGroup`][crate::ArgGroup] [`Id`]s via [`ArgMatches::ids`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " # use clap::{Command, arg, value_parser};"] # [doc = ""] # [doc = " let m = Command::new(\"myprog\")"] # [doc = "     .arg(arg!(--color <when>)"] # [doc = "         .value_parser([\"auto\", \"always\", \"never\"]))"] # [doc = "     .arg(arg!(--config <path>)"] # [doc = "         .value_parser(value_parser!(std::path::PathBuf)))"] # [doc = "     .get_matches_from([\"myprog\", \"--config=config.toml\", \"--color=auto\"]);"] # [doc = " assert_eq!("] # [doc = "     m.ids()"] # [doc = "         .map(|id| id.as_str())"] # [doc = "         .collect::<Vec<_>>(),"] # [doc = "     [\"config\", \"color\"]"] # [doc = " );"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct IdsRef < 'a > { iter : Iter < 'a , Id > , }
    };
}

IdsRef!()