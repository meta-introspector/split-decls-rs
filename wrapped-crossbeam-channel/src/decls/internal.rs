macro_rules! deps {
    () => {
        SelectHandle!();
    };
}

macro_rules! internal {
    () => {
        deps!();
        # [doc = " Crate internals used by the `select!` macro."] # [doc (hidden)] # [cfg (feature = "std")] pub mod internal { pub use crate :: select :: { select , select_timeout , try_select , SelectHandle } ; }
    };
}

internal!()