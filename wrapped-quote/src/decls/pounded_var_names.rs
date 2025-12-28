macro_rules! pounded_var_names {
    () => {
        # [macro_export] # [doc (hidden)] macro_rules ! pounded_var_names { ($ call : ident ! $ extra : tt $ ($ tts : tt) *) => { $ crate :: pounded_var_names_with_context ! { $ call ! $ extra (@ $ ($ tts) *) ($ ($ tts) * @) } } ; }
    };
}

pounded_var_names!();