macro_rules! pounded_var_with_context {
    () => {
        # [macro_export] # [doc (hidden)] macro_rules ! pounded_var_with_context { ($ call : ident ! $ extra : tt $ b1 : tt ($ ($ inner : tt) *)) => { $ crate :: pounded_var_names ! ($ call ! $ extra $ ($ inner) *) ; } ; ($ call : ident ! $ extra : tt $ b1 : tt [$ ($ inner : tt) *]) => { $ crate :: pounded_var_names ! ($ call ! $ extra $ ($ inner) *) ; } ; ($ call : ident ! $ extra : tt $ b1 : tt { $ ($ inner : tt) * }) => { $ crate :: pounded_var_names ! ($ call ! $ extra $ ($ inner) *) ; } ; ($ call : ident ! ($ ($ extra : tt) *) # $ var : ident) => { $ crate ::$ call ! ($ ($ extra) * $ var) ; } ; ($ call : ident ! $ extra : tt $ b1 : tt $ curr : tt) => { } ; }
    };
}

pounded_var_with_context!()