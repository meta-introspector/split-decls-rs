// Generated macro for impl_23 (impl)
macro_rules! Depcrate_attrimpl_23 {
() => {
// Module: crate::attr
// Provides: {"impl_23"}
// Dependencies: {}
impl AttrKind { pub (crate) fn as_str (& self) -> & 'static str { match self { Self :: Clap => "clap" , Self :: StructOpt => "structopt" , Self :: Command => "command" , Self :: Group => "group" , Self :: Arg => "arg" , Self :: Value => "value" , } } }
};
}
