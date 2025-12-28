macro_rules! deps {
    () => {
        LocalsStateAtExit!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        # [extension (pub trait PlaceExt <'tcx >)] impl < 'tcx > Place < 'tcx > { # [doc = " Returns `true` if we can safely ignore borrows of this place."] # [doc = " This is true whenever there is no action that the user can do"] # [doc = " to the place `self` that would invalidate the borrow. This is true"] # [doc = " for borrows of raw pointer dereferents as well as shared references."] fn ignore_borrow (& self , tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , locals_state_at_exit : & LocalsStateAtExit ,) -> bool { if let LocalsStateAtExit :: SomeAreInvalidated { has_storage_dead_or_moved } = locals_state_at_exit { let ignore = ! has_storage_dead_or_moved . contains (self . local) && body . local_decls [self . local] . mutability == Mutability :: Not ; debug ! ("ignore_borrow: local {:?} => {:?}" , self . local , ignore) ; if ignore { return true ; } } for (i , (proj_base , elem)) in self . iter_projections () . enumerate () { if elem == ProjectionElem :: Deref { let ty = proj_base . ty (body , tcx) . ty ; match ty . kind () { ty :: Ref (_ , _ , hir :: Mutability :: Not) if i == 0 => { if body . local_decls [self . local] . is_ref_to_thread_local () { continue ; } return true ; } ty :: RawPtr (..) | ty :: Ref (_ , _ , hir :: Mutability :: Not) => { return true ; } _ => { } } } } false } }
    };
}

impl_206!()