macro_rules! deps {
    () => {
        MemPlaceMeta!();
        OffsetMode!();
        InterpCx!();
        Projectable!();
        ArrayIterator!();
        Machine!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl < 'a , 'tcx , Prov : Provenance , P : Projectable < 'tcx , Prov > > ArrayIterator < 'a , 'tcx , Prov , P > { # [doc = " Should be the same `ecx` on each call, and match the one used to create the iterator."] pub fn next < M : Machine < 'tcx , Provenance = Prov > > (& mut self , ecx : & InterpCx < 'tcx , M > ,) -> InterpResult < 'tcx , Option < (u64 , P) > > { let Some (idx) = self . range . next () else { return interp_ok (None) } ; interp_ok (Some ((idx , self . base . offset_with_meta (self . stride * idx , OffsetMode :: Wrapping , MemPlaceMeta :: None , self . field_layout , ecx ,) ? ,))) } }
    };
}

impl_308!();