// Generated macro for impl_359 (impl)
macro_rules! Depcrate_builder_value_parserimpl_359 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_359"}
// Dependencies: {}
# [doc = " Create a [`ValueParser`] with [`PossibleValuesParser`]"] # [doc = ""] # [doc = " See [`PossibleValuesParser`] for more flexibility in creating the"] # [doc = " [`PossibleValue`][crate::builder::PossibleValue]s."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " let possible = vec![\"always\", \"auto\", \"never\"];"] # [doc = " let mut cmd = clap::Command::new(\"raw\")"] # [doc = "     .arg("] # [doc = "         clap::Arg::new(\"color\")"] # [doc = "             .long(\"color\")"] # [doc = "             .value_parser(possible)"] # [doc = "             .default_value(\"auto\")"] # [doc = "     );"] # [doc = ""] # [doc = " let m = cmd.try_get_matches_from_mut("] # [doc = "     [\"cmd\", \"--color\", \"never\"]"] # [doc = " ).unwrap();"] # [doc = ""] # [doc = " let color: &String = m.get_one(\"color\")"] # [doc = "     .expect(\"default\");"] # [doc = " assert_eq!(color, \"never\");"] # [doc = " ```"] impl < P > From < Vec < P > > for ValueParser where P : Into < super :: PossibleValue > , { fn from (values : Vec < P >) -> Self { let inner = PossibleValuesParser :: from (values) ; Self :: from (inner) } }
};
}
