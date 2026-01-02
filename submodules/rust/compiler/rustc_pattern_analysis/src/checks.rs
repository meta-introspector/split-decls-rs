mkuse!{use crate :: constructor :: Constructor :: * ;}
mkuse!{use crate :: pat_column :: PatternColumn ;}
mkuse!{use crate :: { MatchArm , PatCx } ;}

macro_rules! detect_mixed_deref_pat_ctors_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function detect_mixed_deref_pat_ctors in module {}", module_path!());
    };
}

mkfn!{
    detect_mixed_deref_pat_ctors_introspect!();
    # [doc = " Validate that deref patterns and normal constructors aren't used to match on the same place."] pub (crate) fn detect_mixed_deref_pat_ctors < 'p , Cx : PatCx > (cx : & Cx , arms : & [MatchArm < 'p , Cx >] ,) -> Result < () , Cx :: Error > { let pat_column = PatternColumn :: new (arms) ; detect_mixed_deref_pat_ctors_inner (cx , & pat_column) }
}

macro_rules! detect_mixed_deref_pat_ctors_inner_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function detect_mixed_deref_pat_ctors_inner in module {}", module_path!());
    };
}

mkfn!{
    detect_mixed_deref_pat_ctors_inner_introspect!();
    fn detect_mixed_deref_pat_ctors_inner < 'p , Cx : PatCx > (cx : & Cx , column : & PatternColumn < 'p , Cx > ,) -> Result < () , Cx :: Error > { let Some (ty) = column . head_ty () else { return Ok (()) ; } ; let mut deref_pat = None ; let mut normal_pat = None ; for pat in column . iter () { match pat . ctor () { Wildcard | Opaque (_) => { } DerefPattern (_) => deref_pat = Some (pat) , _ => normal_pat = Some (pat) , } } if let Some (deref_pat) = deref_pat && let Some (normal_pat) = normal_pat { return Err (cx . report_mixed_deref_pat_ctors (deref_pat , normal_pat)) ; } let set = column . analyze_ctors (cx , & ty) ? ; for ctor in set . present { for specialized_column in column . specialize (cx , & ty , & ctor) . iter () { detect_mixed_deref_pat_ctors_inner (cx , specialized_column) ? ; } } Ok (()) }
}