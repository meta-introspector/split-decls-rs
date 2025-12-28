macro_rules! _memoffset_offset_from_unsafe {
    () => {
        # [cfg (not (stable_const))] # [macro_export] # [doc (hidden)] macro_rules ! _memoffset_offset_from_unsafe { ($ field : expr , $ base : expr) => { ($ field as usize) - ($ base as usize) } ; }
    };
}

_memoffset_offset_from_unsafe!();