// Generated macro for Tag (enum)
macro_rules! DepcrateTag {
() => {
// Module: crate
// Provides: {"Tag"}
// Dependencies: {}
# [doc = " Specifies the origin of a format string"] # [derive (PartialEq , Eq , Debug)] pub enum Tag { # [doc = " Defmt-controlled format string for primitive types."] Prim , # [doc = " Format string created by `#[derive(Format)]`."] Derived , # [doc = " Format string created by `defmt::bitflags!`."] Bitflags , # [doc = " A user-defined format string from a `write!` invocation."] Write , # [doc = " An interned string, for use with `{=istr}`."] Str , # [doc = " Defines the global timestamp format."] Timestamp , # [doc = " `static` containing a possible value of a bitflags type."] BitflagsValue , # [doc = " Format string created by `defmt::println!`."] Println , Trace , Debug , Info , Warn , Error , }
};
}
