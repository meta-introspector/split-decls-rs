macro_rules! size_and_align_of {
    () => {
        # [doc = " Extract size and alignment from a TyAndLayout."] # [inline] fn size_and_align_of (ty_and_layout : TyAndLayout < '_ >) -> (Size , Align) { (ty_and_layout . size , ty_and_layout . align . abi) }
    };
}

size_and_align_of!();