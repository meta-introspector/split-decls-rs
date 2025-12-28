macro_rules! deps {
    () => {
        Delegate!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < 'tcx , D : Delegate < 'tcx > > Delegate < 'tcx > for & mut D { fn consume (& mut self , place_with_id : & PlaceWithHirId < 'tcx > , diag_expr_id : HirId) { (* * self) . consume (place_with_id , diag_expr_id) } fn use_cloned (& mut self , place_with_id : & PlaceWithHirId < 'tcx > , diag_expr_id : HirId) { (* * self) . use_cloned (place_with_id , diag_expr_id) } fn borrow (& mut self , place_with_id : & PlaceWithHirId < 'tcx > , diag_expr_id : HirId , bk : ty :: BorrowKind ,) { (* * self) . borrow (place_with_id , diag_expr_id , bk) } fn copy (& mut self , place_with_id : & PlaceWithHirId < 'tcx > , diag_expr_id : HirId) { (* * self) . copy (place_with_id , diag_expr_id) } fn mutate (& mut self , assignee_place : & PlaceWithHirId < 'tcx > , diag_expr_id : HirId) { (* * self) . mutate (assignee_place , diag_expr_id) } fn bind (& mut self , binding_place : & PlaceWithHirId < 'tcx > , diag_expr_id : HirId) { (* * self) . bind (binding_place , diag_expr_id) } fn fake_read (& mut self , place_with_id : & PlaceWithHirId < 'tcx > , cause : FakeReadCause , diag_expr_id : HirId ,) { (* * self) . fake_read (place_with_id , cause , diag_expr_id) } }
    };
}

impl_158!();