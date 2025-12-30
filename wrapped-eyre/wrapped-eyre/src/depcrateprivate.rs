// Generated macro for private (module)
macro_rules! Depcrateprivate {
() => {
// Module: crate
// Provides: {"private"}
// Dependencies: {}
# [doc (hidden)] pub mod private { use crate :: Report ; use alloc :: fmt ; use core :: fmt :: { Arguments , Debug , Display } ; pub use alloc :: format ; pub use core :: format_args ; pub use core :: result :: Result :: Err ; # [doc (hidden)] pub mod kind { pub use crate :: kind :: { AdhocKind , TraitKind } ; pub use crate :: kind :: BoxedKind ; } # [cfg_attr (track_caller , track_caller)] pub fn new_adhoc < M > (message : M) -> Report where M : Display + Debug + Send + Sync + 'static , { Report :: from_adhoc (message) } # [doc (hidden)] # [cold] # [cfg_attr (track_caller , track_caller)] pub fn format_err (args : Arguments < '_ >) -> Report { # [cfg (eyre_no_fmt_arguments_as_str)] let fmt_arguments_as_str : Option < & str > = None ; # [cfg (not (eyre_no_fmt_arguments_as_str))] let fmt_arguments_as_str = args . as_str () ; if let Some (message) = fmt_arguments_as_str { Report :: msg (message) } else { Report :: msg (fmt :: format (args)) } } }
};
}
