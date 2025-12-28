macro_rules! deps {
    () => {
        MemPlaceMeta!();
    };
}

macro_rules! reconstruct_place_meta {
    () => {
        deps!();
        # [doc = " Valtrees don't store the `MemPlaceMeta` that all dynamically sized values have in the interpreter."] # [doc = " This function reconstructs it."] fn reconstruct_place_meta < 'tcx > (layout : TyAndLayout < 'tcx > , valtree : ty :: ValTree < 'tcx > , tcx : TyCtxt < 'tcx > ,) -> MemPlaceMeta { if layout . is_sized () { return MemPlaceMeta :: None ; } let mut last_valtree = valtree ; let tail = tcx . struct_tail_raw (layout . ty , | ty | ty , | | { let branches = last_valtree . unwrap_branch () ; last_valtree = * branches . last () . unwrap () ; debug ! (? branches , ? last_valtree) ; } ,) ; match tail . kind () { ty :: Slice (..) | ty :: Str => { } _ => bug ! ("unsized tail of a valtree must be Slice or Str") , } ; let num_elems = last_valtree . unwrap_branch () . len () ; MemPlaceMeta :: Meta (Scalar :: from_target_usize (num_elems as u64 , & tcx)) }
    };
}

reconstruct_place_meta!()