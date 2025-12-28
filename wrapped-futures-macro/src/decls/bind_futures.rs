macro_rules! bind_futures {
    () => {
        fn bind_futures (fut_exprs : Vec < Expr > , span : Span) -> (Vec < TokenStream2 > , Vec < Ident >) { let mut future_let_bindings = Vec :: with_capacity (fut_exprs . len ()) ; let future_names : Vec < _ > = fut_exprs . into_iter () . enumerate () . map (| (i , expr) | { let name = format_ident ! ("_fut{}" , i , span = span) ; future_let_bindings . push (quote ! { let mut # name = __futures_crate :: future :: maybe_done (# expr) ; let mut # name = unsafe { __futures_crate :: Pin :: new_unchecked (& mut # name) } ; }) ; name }) . collect () ; (future_let_bindings , future_names) }
    };
}

bind_futures!()