macro_rules! deps {
    () => {
        Memory!();
    };
}

macro_rules! check_validity_requirement_lax {
    () => {
        deps!();
        # [doc = " Implements the 'lax' (default) version of the [`check_validity_requirement`] checks; see that"] # [doc = " function for details."] fn check_validity_requirement_lax < 'tcx > (this : TyAndLayout < 'tcx > , cx : & LayoutCx < 'tcx > , init_kind : ValidityRequirement ,) -> Result < bool , & 'tcx LayoutError < 'tcx > > { let scalar_allows_raw_init = move | s : Scalar | -> bool { match init_kind { ValidityRequirement :: Inhabited => { bug ! ("ValidityRequirement::Inhabited should have been handled above") } ValidityRequirement :: Zero => { s . valid_range (cx) . contains (0) } ValidityRequirement :: UninitMitigated0x01Fill => { let mut val : u128 = 0x01 ; for _ in 1 .. s . size (cx) . bytes () { val = (val << 8) | 0x01 ; } s . valid_range (cx) . contains (val) } ValidityRequirement :: Uninit => { bug ! ("ValidityRequirement::Uninit should have been handled above") } } } ; let valid = ! this . is_uninhabited () && match this . backend_repr { BackendRepr :: Scalar (s) => scalar_allows_raw_init (s) , BackendRepr :: ScalarPair (s1 , s2) => { scalar_allows_raw_init (s1) && scalar_allows_raw_init (s2) } BackendRepr :: SimdVector { element : s , count } => count == 0 || scalar_allows_raw_init (s) , BackendRepr :: Memory { .. } => true , } ; if ! valid { return Ok (false) ; } if let Some (pointee) = this . ty . builtin_deref (false) { let pointee = cx . layout_of (pointee) ? ; if pointee . align . abi . bytes () > 1 { return Ok (false) ; } if pointee . size . bytes () > 0 { return Ok (false) ; } } match & this . fields { FieldsShape :: Primitive | FieldsShape :: Union { .. } => { } FieldsShape :: Array { .. } => { } FieldsShape :: Arbitrary { offsets , .. } => { for idx in 0 .. offsets . len () { if ! check_validity_requirement_lax (this . field (cx , idx) , cx , init_kind) ? { return Ok (false) ; } } } } match & this . variants { Variants :: Empty => return Ok (false) , Variants :: Single { .. } => { } Variants :: Multiple { .. } => { } } Ok (true) }
    };
}

check_validity_requirement_lax!();