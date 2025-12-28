macro_rules! deps {
    () => {
        Guard!();
        Entry!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl Entry { # [doc = " Marks this entry as deleted, deferring the actual deallocation to a later iteration."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The entry should be a member of a linked list, and it should not have been deleted."] # [doc = " It should be safe to call `C::finalize` on the entry after the `guard` is dropped, where `C`"] # [doc = " is the associated helper for the linked list."] pub (crate) unsafe fn delete (& self , guard : & Guard) { self . next . fetch_or (1 , Release , guard) ; } }
    };
}

impl_124!();