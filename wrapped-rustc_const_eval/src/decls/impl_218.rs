macro_rules! deps {
    () => {
        Place!();
        Machine!();
        PlacePrinter!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl < 'a , 'tcx , M : Machine < 'tcx > > std :: fmt :: Debug for PlacePrinter < 'a , 'tcx , M > { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self . place { Place :: Local { local , offset , locals_addr } => { debug_assert_eq ! (locals_addr , self . ecx . frame () . locals_addr ()) ; let mut allocs = Vec :: new () ; write ! (fmt , "{local:?}") ? ; if let Some (offset) = offset { write ! (fmt , "+{:#x}" , offset . bytes ()) ? ; } write ! (fmt , ":") ? ; self . ecx . frame () . locals [local] . print (& mut allocs , fmt) ? ; write ! (fmt , ": {:?}" , self . ecx . dump_allocs (allocs . into_iter () . flatten () . collect ())) } Place :: Ptr (mplace) => match mplace . ptr . provenance . and_then (Provenance :: get_alloc_id) { Some (alloc_id) => { write ! (fmt , "by ref {:?}: {:?}" , mplace . ptr , self . ecx . dump_alloc (alloc_id)) } ptr => write ! (fmt , " integral by ref: {ptr:?}") , } , } } }
    };
}

impl_218!();