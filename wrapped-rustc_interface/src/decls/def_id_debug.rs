macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! def_id_debug {
    () => {
        deps!();
        # [doc = " This is a callback from `rustc_hir` as it cannot access the implicit state"] # [doc = " in `rustc_middle` otherwise."] fn def_id_debug (def_id : rustc_hir :: def_id :: DefId , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "DefId({}:{}" , def_id . krate , def_id . index . index ()) ? ; tls :: with_opt (| opt_tcx | { if let Some (tcx) = opt_tcx { write ! (f , " ~ {}" , tcx . def_path_debug_str (def_id)) ? ; } Ok (()) }) ? ; write ! (f , ")") }
    };
}

def_id_debug!()