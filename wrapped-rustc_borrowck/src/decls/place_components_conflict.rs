macro_rules! deps {
    () => {
        AccessDepth!();
        ArtificialField!();
        PlaceConflictBias!();
        Overlap!();
    };
}

macro_rules! place_components_conflict {
    () => {
        deps!();
        # [instrument (level = "debug" , skip (tcx , body))] fn place_components_conflict < 'tcx > (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , borrow_place : Place < 'tcx > , borrow_kind : BorrowKind , access_place : PlaceRef < 'tcx > , access : AccessDepth , bias : PlaceConflictBias ,) -> bool { let borrow_local = borrow_place . local ; let access_local = access_place . local ; assert_eq ! (borrow_local , access_local) ; for ((borrow_place , borrow_c) , & access_c) in iter :: zip (borrow_place . iter_projections () , access_place . projection) { debug ! (? borrow_c , ? access_c) ; match place_projection_conflict (tcx , body , borrow_place , borrow_c , access_c , bias) { Overlap :: Arbitrary => { debug ! ("arbitrary -> conflict") ; return true ; } Overlap :: EqualOrDisjoint => { } Overlap :: Disjoint => { debug ! ("disjoint") ; return false ; } } } if borrow_place . projection . len () > access_place . projection . len () { for (base , elem) in borrow_place . iter_projections () . skip (access_place . projection . len ()) { let base_ty = base . ty (body , tcx) . ty ; match (elem , base_ty . kind () , access) { (_ , _ , Shallow (Some (ArtificialField :: ArrayLength))) | (_ , _ , Shallow (Some (ArtificialField :: FakeBorrow))) => { debug ! ("borrow_conflicts_with_place: implicit field") ; return false ; } (ProjectionElem :: Deref , _ , Shallow (None)) => { debug ! ("borrow_conflicts_with_place: shallow access behind ptr") ; return false ; } (ProjectionElem :: Deref , ty :: Ref (_ , _ , hir :: Mutability :: Not) , _) => { bug ! ("Tracking borrow behind shared reference.") ; } (ProjectionElem :: Deref , ty :: Ref (_ , _ , hir :: Mutability :: Mut) , AccessDepth :: Drop) => { debug ! ("borrow_conflicts_with_place: drop access behind ptr") ; return false ; } (ProjectionElem :: Field { .. } , ty :: Adt (def , _) , AccessDepth :: Drop) => { if def . has_dtor (tcx) { return true ; } } (ProjectionElem :: Deref , _ , Deep) | (ProjectionElem :: Deref , _ , AccessDepth :: Drop) | (ProjectionElem :: Field { .. } , _ , _) | (ProjectionElem :: Index { .. } , _ , _) | (ProjectionElem :: ConstantIndex { .. } , _ , _) | (ProjectionElem :: Subslice { .. } , _ , _) | (ProjectionElem :: OpaqueCast { .. } , _ , _) | (ProjectionElem :: Subtype (_) , _ , _) | (ProjectionElem :: Downcast { .. } , _ , _) | (ProjectionElem :: UnwrapUnsafeBinder (_) , _ , _) => { } } } } if borrow_kind == BorrowKind :: Fake (FakeBorrowKind :: Shallow) && borrow_place . projection . len () < access_place . projection . len () { debug ! ("borrow_conflicts_with_place: shallow borrow") ; false } else { debug ! ("borrow_conflicts_with_place: full borrow, CONFLICT") ; true } }
    };
}

place_components_conflict!();