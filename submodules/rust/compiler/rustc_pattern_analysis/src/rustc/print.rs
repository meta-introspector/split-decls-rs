mkuse!{use std :: fmt ;}
mkuse!{use rustc_abi :: { FieldIdx , VariantIdx } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: { self , AdtDef , Ty , TyCtxt } ;}
mkuse!{use rustc_span :: sym ;}
mkitem!{mkstruct!{# [derive (Clone , Debug)] pub (crate) struct FieldPat { pub (crate) field : FieldIdx , pub (crate) pattern : String , pub (crate) is_wildcard : bool , }}}

macro_rules! start_or_comma_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function start_or_comma in module {}", module_path!());
    };
}

mkfn!{
    start_or_comma_introspect!();
    # [doc = " Returns a closure that will return `\"\"` when called the first time,"] # [doc = " and then return `\", \"` when called any subsequent times."] # [doc = " Useful for printing comma-separated lists."] fn start_or_comma () -> impl FnMut () -> & 'static str { let mut first = true ; move | | { if first { first = false ; "" } else { ", " } } }
}
mkitem!{mkenum!{# [derive (Clone , Debug)] pub (crate) enum EnumInfo < 'tcx > { Enum { adt_def : AdtDef < 'tcx > , variant_index : VariantIdx } , NotEnum , }}}

macro_rules! write_struct_like_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_struct_like in module {}", module_path!());
    };
}

mkfn!{
    write_struct_like_introspect!();
    pub (crate) fn write_struct_like < 'tcx > (f : & mut impl fmt :: Write , tcx : TyCtxt < '_ > , ty : Ty < 'tcx > , enum_info : & EnumInfo < 'tcx > , subpatterns : & [FieldPat] ,) -> fmt :: Result { let variant_and_name = match * enum_info { EnumInfo :: Enum { adt_def , variant_index } => { let variant = adt_def . variant (variant_index) ; let adt_did = adt_def . did () ; let name = if tcx . is_diagnostic_item (sym :: Option , adt_did) || tcx . is_diagnostic_item (sym :: Result , adt_did) { variant . name . to_string () } else { format ! ("{}::{}" , tcx . def_path_str (adt_def . did ()) , variant . name) } ; Some ((variant , name)) } EnumInfo :: NotEnum => ty . ty_adt_def () . and_then (| adt_def | { Some ((adt_def . non_enum_variant () , tcx . def_path_str (adt_def . did ()))) }) , } ; let mut start_or_comma = start_or_comma () ; if let Some ((variant , name)) = & variant_and_name { write ! (f , "{name}") ? ; if variant . ctor . is_none () { write ! (f , " {{ ") ? ; let mut printed = 0 ; for & FieldPat { field , ref pattern , is_wildcard } in subpatterns { if is_wildcard { continue ; } let field_name = variant . fields [field] . name ; write ! (f , "{}{field_name}: {pattern}" , start_or_comma ()) ? ; printed += 1 ; } let is_union = ty . ty_adt_def () . is_some_and (| adt | adt . is_union ()) ; if printed < variant . fields . len () && (! is_union || printed == 0) { write ! (f , "{}.." , start_or_comma ()) ? ; } return write ! (f , " }}") ; } } let num_fields = variant_and_name . as_ref () . map_or (subpatterns . len () , | (v , _) | v . fields . len ()) ; if num_fields != 0 || variant_and_name . is_none () { write ! (f , "(") ? ; for FieldPat { pattern , .. } in subpatterns { write ! (f , "{}{pattern}" , start_or_comma ()) ? ; } if matches ! (ty . kind () , ty :: Tuple (..)) && num_fields == 1 { write ! (f , ",") ? ; } write ! (f , ")") ? ; } Ok (()) }
}

macro_rules! write_ref_like_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_ref_like in module {}", module_path!());
    };
}

mkfn!{
    write_ref_like_introspect!();
    pub (crate) fn write_ref_like < 'tcx > (f : & mut impl fmt :: Write , ty : Ty < 'tcx > , subpattern : & str ,) -> fmt :: Result { match ty . kind () { ty :: Ref (_ , _ , mutbl) => { write ! (f , "&{}" , mutbl . prefix_str ()) ? ; } _ => bug ! ("{ty} is a bad ref pattern type") , } write ! (f , "{subpattern}") }
}

macro_rules! write_slice_like_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_slice_like in module {}", module_path!());
    };
}

mkfn!{
    write_slice_like_introspect!();
    pub (crate) fn write_slice_like (f : & mut impl fmt :: Write , prefix : & [String] , has_dot_dot : bool , suffix : & [String] ,) -> fmt :: Result { let mut start_or_comma = start_or_comma () ; write ! (f , "[") ? ; for p in prefix . iter () { write ! (f , "{}{}" , start_or_comma () , p) ? ; } if has_dot_dot { write ! (f , "{}.." , start_or_comma ()) ? ; } for p in suffix . iter () { write ! (f , "{}{}" , start_or_comma () , p) ? ; } write ! (f , "]") }
}