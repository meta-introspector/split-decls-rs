mkuse!{use std :: assert_matches :: assert_matches ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: { self , CanonicalUserType , TyCtxt } ;}
mkuse!{use tracing :: debug ;}

macro_rules! user_args_applied_to_ty_of_hir_id_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function user_args_applied_to_ty_of_hir_id in module {}", module_path!());
    };
}

mkfn!{
    user_args_applied_to_ty_of_hir_id_introspect!();
    # [doc = " Looks up the type associated with this hir-id and applies the"] # [doc = " user-given generic parameters; the hir-id must map to a suitable"] # [doc = " type."] pub (crate) fn user_args_applied_to_ty_of_hir_id < 'tcx > (tcx : TyCtxt < 'tcx > , typeck_results : & ty :: TypeckResults < 'tcx > , hir_id : hir :: HirId ,) -> Option < CanonicalUserType < 'tcx > > { let user_provided_types = typeck_results . user_provided_types () ; let mut user_ty = * user_provided_types . get (hir_id) ? ; debug ! ("user_subts_applied_to_ty_of_hir_id: user_ty={:?}" , user_ty) ; let ty = typeck_results . node_type (hir_id) ; match ty . kind () { ty :: Adt (adt_def , ..) => { if let ty :: UserTypeKind :: TypeOf (did , _) = & mut user_ty . value . kind { assert_matches ! (tcx . def_kind (* did) , DefKind :: Ctor (..) | DefKind :: Struct | DefKind :: Enum | DefKind :: Union | DefKind :: Variant) ; * did = adt_def . did () ; } Some (user_ty) } ty :: FnDef (..) => Some (user_ty) , _ => bug ! ("ty: {:?} should not have user provided type {:?} recorded " , ty , user_ty) , } }
}