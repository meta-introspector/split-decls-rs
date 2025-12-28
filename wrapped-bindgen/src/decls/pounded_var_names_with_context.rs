macro_rules! pounded_var_names_with_context {
    () => {
        # [macro_export] # [doc (hidden)] macro_rules ! pounded_var_names_with_context { ($ call : ident ! $ extra : tt ($ ($ b1 : tt) *) ($ ($ curr : tt) *)) => { $ ($ crate :: pounded_var_with_context ! ($ call ! $ extra $ b1 $ curr) ;) * } ; }
    };
}

pounded_var_names_with_context!()