macro_rules! deps {
    () => {
        LoanKillsGenerator!();
        PlaceConflictBias!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl < 'tcx > LoanKillsGenerator < '_ , 'tcx > { # [doc = " Records the borrows on the specified place as `killed`. For example, when assigning to a"] # [doc = " local, or on a call's return destination."] fn record_killed_borrows_for_place (& mut self , place : Place < 'tcx > , location : Location) { match place . as_ref () { PlaceRef { local , projection : & [] } | PlaceRef { local , projection : & [ProjectionElem :: Deref] } => { debug ! ("Recording `killed` facts for borrows of local={:?} at location={:?}" , local , location) ; self . record_killed_borrows_for_local (local , location) ; } PlaceRef { local , projection : & [.. , _] } => { debug ! ("Recording `killed` facts for borrows of \
                            innermost projected local={:?} at location={:?}" , local , location) ; if let Some (borrow_indices) = self . borrow_set . local_map . get (& local) { for & borrow_index in borrow_indices { let places_conflict = places_conflict :: places_conflict (self . tcx , self . body , self . borrow_set [borrow_index] . borrowed_place , place , places_conflict :: PlaceConflictBias :: NoOverlap ,) ; if places_conflict { let location_index = self . location_table . mid_index (location) ; self . facts . loan_killed_at . push ((borrow_index , location_index)) ; } } } } } } # [doc = " Records the borrows on the specified local as `killed`."] fn record_killed_borrows_for_local (& mut self , local : Local , location : Location) { if let Some (borrow_indices) = self . borrow_set . local_map . get (& local) { let location_index = self . location_table . mid_index (location) ; self . facts . loan_killed_at . reserve (borrow_indices . len ()) ; for & borrow_index in borrow_indices { self . facts . loan_killed_at . push ((borrow_index , location_index)) ; } } } }
    };
}

impl_241!()