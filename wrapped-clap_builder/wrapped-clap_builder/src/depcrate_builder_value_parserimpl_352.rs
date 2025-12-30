// Generated macro for impl_352 (impl)
macro_rules! Depcrate_builder_value_parserimpl_352 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_352"}
// Dependencies: {}
# [doc = " Create an `i64` [`ValueParser`] from a `N..M` range"] # [doc = ""] # [doc = " See [`RangedI64ValueParser`] for more control over the output type."] # [doc = ""] # [doc = " See also [`RangedU64ValueParser`]"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " let mut cmd = clap::Command::new(\"raw\")"] # [doc = "     .arg("] # [doc = "         clap::Arg::new(\"port\")"] # [doc = "             .long(\"port\")"] # [doc = "             .value_parser(3000..4000)"] # [doc = "             .action(clap::ArgAction::Set)"] # [doc = "             .required(true)"] # [doc = "     );"] # [doc = ""] # [doc = " let m = cmd.try_get_matches_from_mut([\"cmd\", \"--port\", \"3001\"]).unwrap();"] # [doc = " let port: i64 = *m.get_one(\"port\")"] # [doc = "     .expect(\"required\");"] # [doc = " assert_eq!(port, 3001);"] # [doc = " ```"] impl From < std :: ops :: Range < i64 > > for ValueParser { fn from (value : std :: ops :: Range < i64 >) -> Self { let inner = RangedI64ValueParser :: < i64 > :: new () . range (value . start .. value . end) ; Self :: from (inner) } }
};
}
