// Generated macro for Table (struct)
macro_rules! DepcrateTable {
() => {
// Module: crate
// Provides: {"Table"}
// Dependencies: {}
# [doc = " Internal table that holds log levels and maps format strings to indices"] # [derive (Debug , Eq , PartialEq)] pub struct Table { timestamp : Option < TableEntry > , entries : BTreeMap < usize , TableEntry > , bitflags : HashMap < BitflagsKey , Vec < (String , u128) > > , encoding : Encoding , }
};
}
