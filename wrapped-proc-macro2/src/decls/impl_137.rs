macro_rules! deps {
    () => {
        FromStr2!();
        Literal!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        # [cfg (feature = "proc-macro")] impl FromStr2 for proc_macro :: Literal { # [cfg (wrap_proc_macro)] fn valid (src : & str) -> bool { Literal :: from_str_checked (src) . is_ok () } }
    };
}

impl_137!();