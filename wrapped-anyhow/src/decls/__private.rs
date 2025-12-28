macro_rules! deps {
    () => {
        Error!();
        AdhocKind!();
        TraitKind!();
        NotBothDebug!();
        BothDebug!();
        Result!();
        BoxedKind!();
    };
}

macro_rules! __private {
    () => {
        deps!();
        # [doc (hidden)] pub mod __private { use self :: not :: Bool ; use crate :: Error ; use alloc :: fmt ; use core :: fmt :: Arguments ; # [doc (hidden)] pub use crate :: ensure :: { BothDebug , NotBothDebug } ; # [doc (hidden)] pub use alloc :: format ; # [doc (hidden)] pub use core :: result :: Result :: Err ; # [doc (hidden)] pub use core :: { concat , format_args , stringify } ; # [doc (hidden)] pub mod kind { # [doc (hidden)] pub use crate :: kind :: { AdhocKind , TraitKind } ; # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] # [doc (hidden)] pub use crate :: kind :: BoxedKind ; } # [doc (hidden)] # [inline] # [cold] pub fn format_err (args : Arguments) -> Error { # [cfg (anyhow_no_fmt_arguments_as_str)] let fmt_arguments_as_str = None :: < & str > ; # [cfg (not (anyhow_no_fmt_arguments_as_str))] let fmt_arguments_as_str = args . as_str () ; if let Some (message) = fmt_arguments_as_str { Error :: msg (message) } else { Error :: msg (fmt :: format (args)) } } # [doc (hidden)] # [inline] # [cold] # [must_use] pub fn must_use (error : Error) -> Error { error } # [doc (hidden)] # [inline] pub fn not (cond : impl Bool) -> bool { cond . not () } mod not { # [doc (hidden)] pub trait Bool { fn not (self) -> bool ; } impl Bool for bool { # [inline] fn not (self) -> bool { ! self } } impl Bool for & bool { # [inline] fn not (self) -> bool { ! * self } } } }
    };
}

__private!();