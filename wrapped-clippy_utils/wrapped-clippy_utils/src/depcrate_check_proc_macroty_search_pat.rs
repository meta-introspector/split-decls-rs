// Generated macro for ty_search_pat (function)
macro_rules! Depcrate_check_proc_macroty_search_pat {
() => {
// Module: crate::check_proc_macro
// Provides: {"ty_search_pat"}
// Dependencies: {}
fn ty_search_pat (ty : & Ty < '_ >) -> (Pat , Pat) { match ty . kind { TyKind :: Slice (..) | TyKind :: Array (..) => (Pat :: Str ("[") , Pat :: Str ("]")) , TyKind :: Ptr (MutTy { ty , .. }) => (Pat :: Str ("*") , ty_search_pat (ty) . 1) , TyKind :: Ref (_ , MutTy { ty , .. }) => (Pat :: Str ("&") , ty_search_pat (ty) . 1) , TyKind :: FnPtr (fn_ptr) => (if fn_ptr . safety . is_unsafe () { Pat :: Str ("unsafe") } else if fn_ptr . abi != ExternAbi :: Rust { Pat :: Str ("extern") } else { Pat :: MultiStr (& ["fn" , "extern"]) } , match fn_ptr . decl . output { FnRetTy :: DefaultReturn (_) => { if let [.. , ty] = fn_ptr . decl . inputs { ty_search_pat (ty) . 1 } else { Pat :: Str ("(") } } , FnRetTy :: Return (ty) => ty_search_pat (ty) . 1 , } ,) , TyKind :: Never => (Pat :: Str ("!") , Pat :: Str ("!")) , TyKind :: Tup ([]) => (Pat :: Str (")") , Pat :: Str ("(")) , TyKind :: Tup ([ty]) => ty_search_pat (ty) , TyKind :: Tup ([head , .. , tail]) => (ty_search_pat (head) . 0 , ty_search_pat (tail) . 1) , TyKind :: OpaqueDef (..) => (Pat :: Str ("impl") , Pat :: Str ("")) , TyKind :: Path (qpath) => qpath_search_pat (& qpath) , TyKind :: Infer (()) => (Pat :: Str ("_") , Pat :: Str ("_")) , TyKind :: UnsafeBinder (binder_ty) => (Pat :: Str ("unsafe") , ty_search_pat (binder_ty . inner_ty) . 1) , TyKind :: TraitObject (_ , tagged_ptr) if let TraitObjectSyntax :: Dyn = tagged_ptr . tag () => { (Pat :: Str ("dyn") , Pat :: Str ("")) } , _ => (Pat :: Str ("") , Pat :: Str ("")) , } }
};
}
