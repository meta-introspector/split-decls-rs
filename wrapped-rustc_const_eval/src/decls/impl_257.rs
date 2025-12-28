macro_rules! deps {
    () => {
        Machine!();
        Memory!();
        DumpAllocs!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl < 'a , 'tcx , M : Machine < 'tcx > > std :: fmt :: Debug for DumpAllocs < 'a , 'tcx , M > { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { fn write_allocation_track_relocs < 'tcx , Prov : Provenance , Extra , Bytes : AllocBytes > (fmt : & mut std :: fmt :: Formatter < '_ > , tcx : TyCtxt < 'tcx > , allocs_to_print : & mut VecDeque < AllocId > , alloc : & Allocation < Prov , Extra , Bytes > ,) -> std :: fmt :: Result { for alloc_id in alloc . provenance () . provenances () . filter_map (| prov | prov . get_alloc_id ()) { allocs_to_print . push_back (alloc_id) ; } write ! (fmt , "{}" , display_allocation (tcx , alloc)) } let mut allocs_to_print : VecDeque < _ > = self . allocs . iter () . copied () . collect () ; let mut allocs_printed = FxHashSet :: default () ; while let Some (id) = allocs_to_print . pop_front () { if ! allocs_printed . insert (id) { continue ; } write ! (fmt , "{id:?}") ? ; match self . ecx . memory . alloc_map . get (id) { Some ((kind , alloc)) => { write ! (fmt , " ({kind}, ") ? ; write_allocation_track_relocs (& mut * fmt , * self . ecx . tcx , & mut allocs_to_print , alloc ,) ? ; } None => { match self . ecx . tcx . try_get_global_alloc (id) { Some (GlobalAlloc :: Memory (alloc)) => { write ! (fmt , " (unchanged global, ") ? ; write_allocation_track_relocs (& mut * fmt , * self . ecx . tcx , & mut allocs_to_print , alloc . inner () ,) ? ; } Some (GlobalAlloc :: Function { instance , .. }) => { write ! (fmt , " (fn: {instance})") ? ; } Some (GlobalAlloc :: VTable (ty , dyn_ty)) => { write ! (fmt , " (vtable: impl {dyn_ty} for {ty})") ? ; } Some (GlobalAlloc :: TypeId { ty }) => { write ! (fmt , " (typeid for {ty})") ? ; } Some (GlobalAlloc :: Static (did)) => { write ! (fmt , " (static: {})" , self . ecx . tcx . def_path_str (did)) ? ; } None => { write ! (fmt , " (deallocated)") ? ; } } } } writeln ! (fmt) ? ; } Ok (()) } }
    };
}

impl_257!();