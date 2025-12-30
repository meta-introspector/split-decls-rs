// Generated macro for signature_help_for_tuple_struct_pat (function)
macro_rules! Depcrate_signature_helpsignature_help_for_tuple_struct_pat {
() => {
// Module: crate::signature_help
// Provides: {"signature_help_for_tuple_struct_pat"}
// Dependencies: {}
fn signature_help_for_tuple_struct_pat (sema : & Semantics < '_ , RootDatabase > , pat : ast :: TupleStructPat , token : SyntaxToken , edition : Edition , display_target : DisplayTarget ,) -> Option < SignatureHelp > { let path = pat . path () ? ; let path_res = sema . resolve_path (& path) ? ; let mut res = SignatureHelp { doc : None , signature : String :: new () , parameters : vec ! [] , active_parameter : None , } ; let db = sema . db ; let fields : Vec < _ > = if let PathResolution :: Def (ModuleDef :: Variant (variant)) = path_res { let en = variant . parent_enum (db) ; res . doc = en . docs (db) ; format_to ! (res . signature , "enum {}::{} (" , en . name (db) . display (db , edition) , variant . name (db) . display (db , edition)) ; variant . fields (db) } else { let adt = match path_res { PathResolution :: SelfType (imp) => imp . self_ty (db) . as_adt () ? , PathResolution :: Def (ModuleDef :: Adt (adt)) => adt , _ => return None , } ; match adt { hir :: Adt :: Struct (it) => { res . doc = it . docs (db) ; format_to ! (res . signature , "struct {} (" , it . name (db) . display (db , edition)) ; it . fields (db) } _ => return None , } } ; Some (signature_help_for_tuple_pat_ish (db , res , pat . syntax () , token , pat . fields () , fields . into_iter () . map (| it | it . ty (db) . to_type (db)) , display_target ,)) }
};
}
