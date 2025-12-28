macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! dep_kind_debug {
    () => {
        deps!();
        # [doc = " This is a callback from `rustc_query_system` as it cannot access the implicit state"] # [doc = " in `rustc_middle` otherwise."] pub fn dep_kind_debug (kind : DepKind , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { tls :: with_opt (| opt_tcx | { if let Some (tcx) = opt_tcx { write ! (f , "{}" , tcx . dep_kind_info (kind) . name) } else { default_dep_kind_debug (kind , f) } }) }
    };
}

dep_kind_debug!()