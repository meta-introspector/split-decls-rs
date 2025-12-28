macro_rules! transmute_copy_by_val {
    () => {
        # [doc = " # Safety"] # [doc = ""] # [doc = " This function has the same safety requirements as [`core::mem::transmute_copy`]."] # [doc = ""] # [doc = " Since this is a by-value transmutation, it copies the bits from the source value"] # [doc = " into the destination value, then forgets the original, as with the [`core::mem::transmute`]."] # [inline] # [must_use] pub (crate) const unsafe fn transmute_copy_by_val < Src , Dst > (src : Src) -> Dst { # [repr (C)] union ConstHack < Src , Dst > { src : ManuallyDrop < Src > , dst : ManuallyDrop < Dst > , } static_assert ! (Src , Dst => mem :: size_of ::< Src > () >= mem :: size_of ::< Dst > ()) ; ManuallyDrop :: into_inner (unsafe { ConstHack :: < Src , Dst > { src : ManuallyDrop :: new (src) } . dst }) }
    };
}

transmute_copy_by_val!()