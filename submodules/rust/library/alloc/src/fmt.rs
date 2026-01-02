mkuse!{# [stable (feature = "fmt_flags_align" , since = "1.28.0")] pub use core :: fmt :: Alignment ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: fmt :: Error ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: fmt :: { Arguments , write } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: fmt :: { Binary , Octal } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: fmt :: { Debug , Display } ;}
mkuse!{# [unstable (feature = "formatting_options" , issue = "118117")] pub use core :: fmt :: { DebugAsHex , FormattingOptions , Sign } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: fmt :: { DebugList , DebugMap , DebugSet , DebugStruct , DebugTuple } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: fmt :: { Formatter , Result , Write } ;}
mkuse!{# [unstable (feature = "debug_closure_helpers" , issue = "117729")] pub use core :: fmt :: { FromFn , from_fn } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: fmt :: { LowerExp , UpperExp } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: fmt :: { LowerHex , Pointer , UpperHex } ;}
mkuse!{# [cfg (not (no_global_oom_handling))] use crate :: string ;}

macro_rules! format_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function format in module {}", module_path!());
    };
}

mkfn!{
    format_introspect!();
    # [doc = " Takes an [`Arguments`] struct and returns the resulting formatted string."] # [doc = ""] # [doc = " The [`Arguments`] instance can be created with the [`format_args!`] macro."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::fmt;"] # [doc = ""] # [doc = " let s = fmt::format(format_args!(\"Hello, {}!\", \"world\"));"] # [doc = " assert_eq!(s, \"Hello, world!\");"] # [doc = " ```"] # [doc = ""] # [doc = " Please note that using [`format!`] might be preferable."] # [doc = " Example:"] # [doc = ""] # [doc = " ```"] # [doc = " let s = format!(\"Hello, {}!\", \"world\");"] # [doc = " assert_eq!(s, \"Hello, world!\");"] # [doc = " ```"] # [doc = ""] # [doc = " [`format_args!`]: core::format_args"] # [doc = " [`format!`]: crate::format"] # [cfg (not (no_global_oom_handling))] # [must_use] # [stable (feature = "rust1" , since = "1.0.0")] # [inline] pub fn format (args : Arguments < '_ >) -> string :: String { fn format_inner (args : Arguments < '_ >) -> string :: String { let capacity = args . estimated_capacity () ; let mut output = string :: String :: with_capacity (capacity) ; output . write_fmt (args) . expect ("a formatting trait implementation returned an error when the underlying stream did not") ; output } args . as_str () . map_or_else (| | format_inner (args) , crate :: borrow :: ToOwned :: to_owned) }
}