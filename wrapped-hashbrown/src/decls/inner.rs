macro_rules! inner {
    () => {
        # [cfg (not (any (feature = "nightly" , feature = "allocator-api2")))] mod inner { use crate :: alloc :: alloc :: { alloc , dealloc , Layout } ; use core :: ptr :: NonNull ; # [allow (clippy :: missing_safety_doc)] pub unsafe trait Allocator { fn allocate (& self , layout : Layout) -> Result < NonNull < [u8] > , () > ; unsafe fn deallocate (& self , ptr : NonNull < u8 > , layout : Layout) ; } # [derive (Copy , Clone)] pub struct Global ; unsafe impl Allocator for Global { # [inline] fn allocate (& self , layout : Layout) -> Result < NonNull < [u8] > , () > { match unsafe { NonNull :: new (alloc (layout)) } { Some (data) => { Ok (unsafe { NonNull :: new_unchecked (core :: ptr :: slice_from_raw_parts_mut (data . as_ptr () , layout . size () ,)) }) } None => Err (()) , } } # [inline] unsafe fn deallocate (& self , ptr : NonNull < u8 > , layout : Layout) { dealloc (ptr . as_ptr () , layout) ; } } impl Default for Global { # [inline] fn default () -> Self { Global } } pub (crate) fn do_alloc < A : Allocator > (alloc : & A , layout : Layout) -> Result < NonNull < [u8] > , () > { alloc . allocate (layout) } }
    };
}

inner!();