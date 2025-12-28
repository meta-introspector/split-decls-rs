macro_rules! _memoffset__offset_of_union_impl {
    () => {
        # [cfg (stable_offset_of)] # [macro_export (local_inner_macros)] # [doc (hidden)] macro_rules ! _memoffset__offset_of_union_impl { ($ parent : path , $ field : tt) => { { $ crate :: __priv :: mem :: offset_of ! ($ parent , $ field) } } ; }
    };
}

_memoffset__offset_of_union_impl!()