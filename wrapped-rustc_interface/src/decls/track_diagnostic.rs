macro_rules! track_diagnostic {
    () => {
        # [doc = " This is a callback from `rustc_errors` as it cannot access the implicit state"] # [doc = " in `rustc_middle` otherwise. It is used when diagnostic messages are"] # [doc = " emitted and stores them in the current query, if there is one."] fn track_diagnostic < R > (diagnostic : DiagInner , f : & mut dyn FnMut (DiagInner) -> R) -> R { tls :: with_context_opt (| icx | { if let Some (icx) = icx { icx . tcx . dep_graph . record_diagnostic (QueryCtxt :: new (icx . tcx) , & diagnostic) ; let icx = tls :: ImplicitCtxt { task_deps : TaskDepsRef :: Ignore , .. icx . clone () } ; tls :: enter_context (& icx , move | | (* f) (diagnostic)) } else { (* f) (diagnostic) } }) }
    };
}

track_diagnostic!()