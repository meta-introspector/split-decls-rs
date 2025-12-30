// Generated macro for RawStrError (enum)
macro_rules! DepcrateRawStrError {
() => {
// Module: crate
// Provides: {"RawStrError"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord)] pub enum RawStrError { # [doc = " Non `#` characters exist between `r` and `\"`, e.g. `r##~\"abcde\"##`"] InvalidStarter { bad_char : char } , # [doc = " The string was not terminated, e.g. `r###\"abcde\"##`."] # [doc = " `possible_terminator_offset` is the number of characters after `r` or"] # [doc = " `br` where they may have intended to terminate it."] NoTerminator { expected : u32 , found : u32 , possible_terminator_offset : Option < u32 > } , # [doc = " More than 255 `#`s exist."] TooManyDelimiters { found : u32 } , }
};
}
