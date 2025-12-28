macro_rules! deps {
    () => {
        LocalUseMap!();
        LocalUseMapBuild!();
    };
}

macro_rules! impl_445 {
    () => {
        deps!();
        impl LocalUseMap { pub (crate) fn build (live_locals : & [Local] , location_map : & DenseLocationMap , body : & Body < '_ > ,) -> Self { let nones = IndexVec :: from_elem (None , & body . local_decls) ; let mut local_use_map = LocalUseMap { first_def_at : nones . clone () , first_use_at : nones . clone () , first_drop_at : nones , appearances : IndexVec :: new () , } ; if live_locals . is_empty () { return local_use_map ; } let mut locals_with_use_data : IndexVec < Local , bool > = IndexVec :: from_elem (false , & body . local_decls) ; live_locals . iter () . for_each (| & local | locals_with_use_data [local] = true) ; LocalUseMapBuild { local_use_map : & mut local_use_map , location_map , locals_with_use_data } . visit_body (body) ; local_use_map } pub (crate) fn defs (& self , local : Local) -> impl Iterator < Item = PointIndex > { appearances_iter (self . first_def_at [local] , & self . appearances) . map (move | aa | self . appearances [aa] . point_index) } pub (crate) fn uses (& self , local : Local) -> impl Iterator < Item = PointIndex > { appearances_iter (self . first_use_at [local] , & self . appearances) . map (move | aa | self . appearances [aa] . point_index) } pub (crate) fn drops (& self , local : Local) -> impl Iterator < Item = PointIndex > { appearances_iter (self . first_drop_at [local] , & self . appearances) . map (move | aa | self . appearances [aa] . point_index) } }
    };
}

impl_445!()