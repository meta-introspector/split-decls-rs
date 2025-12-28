macro_rules! deps {
    () => {
        DebugInfoOffset!();
        DebugInfoOffsetLocation!();
        BuilderMethods!();
    };
}

macro_rules! calculate_debuginfo_offset {
    () => {
        deps!();
        fn calculate_debuginfo_offset < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > , L : DebugInfoOffsetLocation < 'tcx , Bx > , > (bx : & mut Bx , projection : & [mir :: PlaceElem < 'tcx >] , base : L ,) -> DebugInfoOffset < L > { let mut direct_offset = Size :: ZERO ; let mut indirect_offsets = vec ! [] ; let mut place = base ; for elem in projection { match * elem { mir :: ProjectionElem :: Deref => { indirect_offsets . push (Size :: ZERO) ; place = place . deref (bx) ; } mir :: ProjectionElem :: Field (field , _) => { let offset = indirect_offsets . last_mut () . unwrap_or (& mut direct_offset) ; * offset += place . layout () . fields . offset (field . index ()) ; place = place . project_field (bx , field) ; } mir :: ProjectionElem :: Downcast (_ , variant) => { place = place . downcast (bx , variant) ; } mir :: ProjectionElem :: ConstantIndex { offset : index , min_length : _ , from_end : false , } => { let offset = indirect_offsets . last_mut () . unwrap_or (& mut direct_offset) ; let FieldsShape :: Array { stride , count : _ } = place . layout () . fields else { bug ! ("ConstantIndex on non-array type {:?}" , place . layout ()) } ; * offset += stride * index ; place = place . project_constant_index (bx , index) ; } _ => { assert ! (! elem . can_use_in_debuginfo ()) ; bug ! ("unsupported var debuginfo projection `{:?}`" , projection) } } } DebugInfoOffset { direct_offset , indirect_offsets , result : place } }
    };
}

calculate_debuginfo_offset!()