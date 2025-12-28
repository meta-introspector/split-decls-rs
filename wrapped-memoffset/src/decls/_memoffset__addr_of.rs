macro_rules! _memoffset__addr_of {
    () => {
        # [cfg (not (raw_ref_macros))] # [macro_export] # [doc (hidden)] macro_rules ! _memoffset__addr_of { ($ path : expr) => { { &$ path as * const _ } } ; }
    };
}

_memoffset__addr_of!()