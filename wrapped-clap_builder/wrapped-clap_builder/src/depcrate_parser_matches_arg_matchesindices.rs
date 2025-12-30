// Generated macro for Indices (struct)
macro_rules! Depcrate_parser_matches_arg_matchesIndices {
() => {
// Module: crate::parser::matches::arg_matches
// Provides: {"Indices"}
// Dependencies: {}
# [doc = " Iterate over indices for where an argument appeared when parsing, via [`ArgMatches::indices_of`]"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " # use clap::{Command, Arg, ArgAction};"] # [doc = " let m = Command::new(\"myapp\")"] # [doc = "     .arg(Arg::new(\"output\")"] # [doc = "         .short('o')"] # [doc = "         .num_args(1..)"] # [doc = "         .action(ArgAction::Set))"] # [doc = "     .get_matches_from(vec![\"myapp\", \"-o\", \"val1\", \"val2\"]);"] # [doc = ""] # [doc = " let mut indices = m.indices_of(\"output\").unwrap();"] # [doc = ""] # [doc = " assert_eq!(indices.next(), Some(2));"] # [doc = " assert_eq!(indices.next(), Some(3));"] # [doc = " assert_eq!(indices.next(), None);"] # [doc = " ```"] # [doc = " [`ArgMatches::indices_of`]: ArgMatches::indices_of()"] # [derive (Clone , Debug)] pub struct Indices < 'a > { iter : Cloned < Iter < 'a , usize > > , len : usize , }
};
}
