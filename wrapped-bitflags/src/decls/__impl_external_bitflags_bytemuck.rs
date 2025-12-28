macro_rules! deps {
    () => {
        Flag!();
    };
}

macro_rules! __impl_external_bitflags_bytemuck {
    () => {
        deps!();
        # [macro_export] # [doc (hidden)] # [cfg (not (feature = "bytemuck"))] macro_rules ! __impl_external_bitflags_bytemuck { ($ InternalBitFlags : ident : $ T : ty , $ PublicBitFlags : ident { $ ($ (# [$ inner : ident $ ($ args : tt) *]) * const $ Flag : tt ;) * }) => { } ; }
    };
}

__impl_external_bitflags_bytemuck!();