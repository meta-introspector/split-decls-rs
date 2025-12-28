macro_rules! deps {
    () => {
        DefUse!();
        AccessFactsExtractor!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        impl < 'a , 'tcx > Visitor < 'tcx > for AccessFactsExtractor < 'a , 'tcx > { fn visit_local (& mut self , local : Local , context : PlaceContext , location : Location) { match def_use :: categorize (context) { Some (DefUse :: Def) => { debug ! ("AccessFactsExtractor - emit def") ; self . facts . var_defined_at . push ((local , self . location_to_index (location))) ; } Some (DefUse :: Use) => { debug ! ("AccessFactsExtractor - emit use") ; self . facts . var_used_at . push ((local , self . location_to_index (location))) ; } Some (DefUse :: Drop) => { debug ! ("AccessFactsExtractor - emit drop") ; self . facts . var_dropped_at . push ((local , self . location_to_index (location))) ; } _ => () , } } fn visit_place (& mut self , place : & Place < 'tcx > , context : PlaceContext , location : Location) { self . super_place (place , context , location) ; match context { PlaceContext :: NonMutatingUse (_) | PlaceContext :: MutatingUse (MutatingUseContext :: Borrow) => { let path = match self . move_data . rev_lookup . find (place . as_ref ()) { LookupResult :: Exact (path) | LookupResult :: Parent (Some (path)) => path , _ => { return ; } } ; debug ! ("AccessFactsExtractor - emit path access ({path:?}, {location:?})") ; self . facts . path_accessed_at_base . push ((path , self . location_to_index (location))) ; } _ => { } } } }
    };
}

impl_231!();