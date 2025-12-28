macro_rules! ValueSource {
    () => {
        # [doc = " Origin of the argument's value"] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] # [non_exhaustive] pub enum ValueSource { # [doc = " Value came [`Arg::default_value`][crate::Arg::default_value]"] DefaultValue , # [doc = " Value came [`Arg::env`][crate::Arg::env]"] EnvVariable , # [doc = " Value was passed in on the command-line"] CommandLine , }
    };
}

ValueSource!()