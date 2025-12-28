macro_rules! deps {
    () => {
        LocalValue!();
        MemPlaceMeta!();
        Immediate!();
        LocalState!();
        Operand!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl < 'tcx , Prov : Provenance > LocalState < 'tcx , Prov > { pub (super) fn print (& self , allocs : & mut Vec < Option < AllocId > > , fmt : & mut std :: fmt :: Formatter < '_ > ,) -> std :: fmt :: Result { match self . value { LocalValue :: Dead => write ! (fmt , " is dead") ? , LocalValue :: Live (Operand :: Immediate (Immediate :: Uninit)) => { write ! (fmt , " is uninitialized") ? } LocalValue :: Live (Operand :: Indirect (mplace)) => { write ! (fmt , " by {} ref {:?}:" , match mplace . meta { MemPlaceMeta :: Meta (meta) => format ! (" meta({meta:?})") , MemPlaceMeta :: None => String :: new () , } , mplace . ptr ,) ? ; allocs . extend (mplace . ptr . provenance . map (Provenance :: get_alloc_id)) ; } LocalValue :: Live (Operand :: Immediate (Immediate :: Scalar (val))) => { write ! (fmt , " {val:?}") ? ; if let Scalar :: Ptr (ptr , _size) = val { allocs . push (ptr . provenance . get_alloc_id ()) ; } } LocalValue :: Live (Operand :: Immediate (Immediate :: ScalarPair (val1 , val2))) => { write ! (fmt , " ({val1:?}, {val2:?})") ? ; if let Scalar :: Ptr (ptr , _size) = val1 { allocs . push (ptr . provenance . get_alloc_id ()) ; } if let Scalar :: Ptr (ptr , _size) = val2 { allocs . push (ptr . provenance . get_alloc_id ()) ; } } } Ok (()) } }
    };
}

impl_327!();