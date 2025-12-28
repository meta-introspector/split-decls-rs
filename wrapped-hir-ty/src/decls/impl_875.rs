macro_rules! deps {
    () => {
        ProjectionId!();
        ProjectionStore!();
        PlaceElem!();
    };
}

macro_rules! impl_875 {
    () => {
        deps!();
        impl < 'db > ProjectionStore < 'db > { pub fn shrink_to_fit (& mut self) { self . id_to_proj . shrink_to_fit () ; self . proj_to_id . shrink_to_fit () ; } pub fn intern_if_exist (& self , projection : & [PlaceElem < 'db >]) -> Option < ProjectionId > { self . proj_to_id . get (projection) . copied () } pub fn intern (& mut self , projection : Box < [PlaceElem < 'db >] >) -> ProjectionId { let new_id = ProjectionId (self . proj_to_id . len () as u32) ; match self . proj_to_id . entry (projection) { Entry :: Occupied (id) => * id . get () , Entry :: Vacant (e) => { let key_clone = e . key () . clone () ; e . insert (new_id) ; self . id_to_proj . insert (new_id , key_clone) ; new_id } } } }
    };
}

impl_875!();