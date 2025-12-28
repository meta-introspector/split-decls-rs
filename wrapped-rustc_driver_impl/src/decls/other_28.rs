macro_rules! other_28 {
    () => {
        # [doc = " Prints version information"] # [doc = ""] # [doc = " NOTE: this is a macro to support drivers built at a different time than the main `rustc_driver` crate."] pub macro version ($ early_dcx : expr , $ binary : literal , $ matches : expr) { fn unw (x : Option <& str >) -> & str { x . unwrap_or ("unknown") } $ crate :: version_at_macro_invocation ($ early_dcx , $ binary , $ matches , unw (option_env ! ("CFG_VERSION")) , unw (option_env ! ("CFG_VER_HASH")) , unw (option_env ! ("CFG_VER_DATE")) , unw (option_env ! ("CFG_RELEASE")) ,) }
    };
}

other_28!()