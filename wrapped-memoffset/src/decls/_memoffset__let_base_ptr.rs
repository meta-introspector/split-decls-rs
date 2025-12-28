macro_rules! _memoffset__let_base_ptr {
    () => {
        # [cfg (not (maybe_uninit))] # [macro_export] # [doc (hidden)] macro_rules ! _memoffset__let_base_ptr { ($ name : ident , $ type : ty) => { let $ name = $ crate :: __priv :: mem :: align_of ::<$ type > () as * const $ type ; } ; }
    };
}

_memoffset__let_base_ptr!()