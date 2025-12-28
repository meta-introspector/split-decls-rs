macro_rules! deps {
    () => {
        CompileTimeInterpCx!();
        Immediate!();
        MPlaceTy!();
    };
}

macro_rules! valtree_into_mplace {
    () => {
        deps!();
        # [instrument (skip (ecx) , level = "debug")] fn valtree_into_mplace < 'tcx > (ecx : & mut CompileTimeInterpCx < 'tcx > , place : & MPlaceTy < 'tcx > , valtree : ty :: ValTree < 'tcx > ,) { let ty = place . layout . ty ; match ty . kind () { ty :: FnDef (_ , _) => { } ty :: Bool | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Char | ty :: RawPtr (..) => { let scalar_int = valtree . unwrap_leaf () ; debug ! ("writing trivial valtree {:?} to place {:?}" , scalar_int , place) ; ecx . write_immediate (Immediate :: Scalar (scalar_int . into ()) , place) . unwrap () ; } ty :: Ref (_ , inner_ty , _) => { let imm = valtree_to_ref (ecx , valtree , * inner_ty) ; debug ! (? imm) ; ecx . write_immediate (imm , place) . unwrap () ; } ty :: Adt (_ , _) | ty :: Tuple (_) | ty :: Array (_ , _) | ty :: Str | ty :: Slice (_) => { let branches = valtree . unwrap_branch () ; let (place_adjusted , branches , variant_idx) = match ty . kind () { ty :: Adt (def , _) if def . is_enum () => { let scalar_int = branches [0] . unwrap_leaf () ; let variant_idx = VariantIdx :: from_u32 (scalar_int . to_u32 ()) ; let variant = def . variant (variant_idx) ; debug ! (? variant) ; (ecx . project_downcast (place , variant_idx) . unwrap () , & branches [1 ..] , Some (variant_idx) ,) } _ => (place . clone () , branches , None) , } ; debug ! (? place_adjusted , ? branches) ; for (i , inner_valtree) in branches . iter () . enumerate () { debug ! (? i , ? inner_valtree) ; let place_inner = match ty . kind () { ty :: Str | ty :: Slice (_) | ty :: Array (..) => { ecx . project_index (place , i as u64) . unwrap () } _ => ecx . project_field (& place_adjusted , FieldIdx :: from_usize (i)) . unwrap () , } ; debug ! (? place_inner) ; valtree_into_mplace (ecx , & place_inner , * inner_valtree) ; dump_place (ecx , & place_inner) ; } debug ! ("dump of place_adjusted:") ; dump_place (ecx , & place_adjusted) ; if let Some (variant_idx) = variant_idx { ecx . write_discriminant (variant_idx , place) . unwrap () ; } debug ! ("dump of place after writing discriminant:") ; dump_place (ecx , place) ; } _ => bug ! ("shouldn't have created a ValTree for {:?}" , ty) , } }
    };
}

valtree_into_mplace!();