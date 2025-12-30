// Generated macro for Frame (struct)
macro_rules! Depcrate_frameFrame {
() => {
// Module: crate::frame
// Provides: {"Frame"}
// Dependencies: {}
# [doc = " A log frame"] # [derive (Debug , PartialEq)] pub struct Frame < 't > { table : & 't Table , level : Option < Level > , index : u64 , timestamp_format : Option < & 't str > , timestamp_args : Vec < Arg < 't > > , format : & 't str , args : Vec < Arg < 't > > , }
};
}
