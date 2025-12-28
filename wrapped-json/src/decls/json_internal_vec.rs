macro_rules! json_internal_vec {
    () => {
        # [macro_export] # [doc (hidden)] macro_rules ! json_internal_vec { ($ ($ content : tt) *) => { vec ! [$ ($ content) *] } ; }
    };
}

json_internal_vec!();