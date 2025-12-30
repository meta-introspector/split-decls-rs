// Generated macro for FixedTimespan (struct)
macro_rules! Depcrate_timezone_implFixedTimespan {
() => {
// Module: crate::timezone_impl
// Provides: {"FixedTimespan"}
// Dependencies: {}
# [doc = " An Offset that applies for a period of time"] # [doc = ""] # [doc = " For example, [`::US::Eastern`] is composed of at least two"] # [doc = " `FixedTimespan`s: `EST` and `EDT`, that are variously in effect."] # [derive (Copy , Clone , PartialEq , Eq)] pub struct FixedTimespan { # [doc = " The base offset from UTC; this usually doesn't change unless the government changes something"] pub offset : i32 , # [doc = " The name of this timezone, for example the difference between `EDT`/`EST`"] pub name : & 'static str , }
};
}
