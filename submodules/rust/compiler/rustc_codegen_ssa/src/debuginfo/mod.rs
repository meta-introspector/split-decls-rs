mkuse!{use rustc_abi :: { Integer , Primitive , Size , TagEncoding , Variants } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: layout :: { IntegerExt , PrimitiveExt , TyAndLayout } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkmod!{type_names, { 
                getname!(type_names);
                getsrc!(type_names);
                getpath!(type_names);
                get_deps!(type_names);
                get_crates!(type_names);
                mkinclude!(type_names);
                 
            }}

macro_rules! wants_c_like_enum_debuginfo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function wants_c_like_enum_debuginfo in module {}", module_path!());
    };
}

mkfn!{
    wants_c_like_enum_debuginfo_introspect!();
    # [doc = " Returns true if we want to generate a DW_TAG_enumeration_type description for"] # [doc = " this instead of a DW_TAG_struct_type with DW_TAG_variant_part."] # [doc = ""] # [doc = " NOTE: This is somewhat inconsistent right now: For empty enums and enums with a single"] # [doc = "       fieldless variant, we generate DW_TAG_struct_type, although a"] # [doc = "       DW_TAG_enumeration_type would be a better fit."] pub fn wants_c_like_enum_debuginfo < 'tcx > (tcx : TyCtxt < 'tcx > , enum_type_and_layout : TyAndLayout < 'tcx > ,) -> bool { match enum_type_and_layout . ty . kind () { ty :: Adt (adt_def , _) => { if ! adt_def . is_enum () { return false ; } if type_names :: cpp_like_debuginfo (tcx) && tag_base_type_opt (tcx , enum_type_and_layout) . map (| ty | ty . primitive_size (tcx) . bits ()) == Some (128) { return false ; } match adt_def . variants () . len () { 0 => false , 1 => { enum_type_and_layout . size != Size :: ZERO && adt_def . all_fields () . count () == 0 } _ => { adt_def . all_fields () . count () == 0 } } } _ => false , } }
}

macro_rules! tag_base_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function tag_base_type in module {}", module_path!());
    };
}

mkfn!{
    tag_base_type_introspect!();
    # [doc = " Extract the type with which we want to describe the tag of the given enum or coroutine."] pub fn tag_base_type < 'tcx > (tcx : TyCtxt < 'tcx > , enum_type_and_layout : TyAndLayout < 'tcx >) -> Ty < 'tcx > { tag_base_type_opt (tcx , enum_type_and_layout) . unwrap_or_else (| | { bug ! ("tag_base_type() called for enum without tag: {:?}" , enum_type_and_layout) }) }
}

macro_rules! tag_base_type_opt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function tag_base_type_opt in module {}", module_path!());
    };
}

mkfn!{
    tag_base_type_opt_introspect!();
    fn tag_base_type_opt < 'tcx > (tcx : TyCtxt < 'tcx > , enum_type_and_layout : TyAndLayout < 'tcx > ,) -> Option < Ty < 'tcx > > { assert ! (match enum_type_and_layout . ty . kind () { ty :: Coroutine (..) => true , ty :: Adt (adt_def , _) => adt_def . is_enum () , _ => false , }) ; match enum_type_and_layout . layout . variants () { Variants :: Single { .. } | Variants :: Empty => None , Variants :: Multiple { tag_encoding : TagEncoding :: Niche { .. } , tag , .. } => { Some (match tag . primitive () { Primitive :: Int (t , _) => t , Primitive :: Float (f) => Integer :: from_size (f . size ()) . unwrap () , Primitive :: Pointer (_) => { tcx . data_layout . ptr_sized_integer () } } . to_ty (tcx , false) ,) } Variants :: Multiple { tag_encoding : TagEncoding :: Direct , tag , .. } => { Some (tag . primitive () . to_ty (tcx)) } } }
}