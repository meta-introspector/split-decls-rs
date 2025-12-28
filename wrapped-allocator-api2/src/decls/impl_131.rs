macro_rules! deps {
    () => {
        IntoIter!();
        Allocator!();
        RawVec!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < T , A : Allocator > Drop for IntoIter < T , A > { fn drop (& mut self) { struct DropGuard < 'a , T , A : Allocator > (& 'a mut IntoIter < T , A >) ; impl < T , A : Allocator > Drop for DropGuard < '_ , T , A > { fn drop (& mut self) { unsafe { let alloc = ManuallyDrop :: take (& mut self . 0 . alloc) ; let _ = RawVec :: from_raw_parts_in (self . 0 . buf . as_ptr () , self . 0 . cap , alloc) ; } } } let guard = DropGuard (self) ; unsafe { ptr :: drop_in_place (guard . 0 . as_raw_mut_slice ()) ; } } }
    };
}

impl_131!()