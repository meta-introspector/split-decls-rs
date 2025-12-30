// Generated macro for impl_65 (impl)
macro_rules! Depcrateimpl_65 {
() => {
// Module: crate
// Provides: {"impl_65"}
// Dependencies: {}
impl MacroReplace { fn new (name : & 'static str) -> Self { let norm_name = base_name (name) ; Self { fn_name : name , norm_name : norm_name . to_owned () , error : None , } } fn finish (self) -> syn :: Result < () > { match self . error { Some (e) => Err (e) , None => Ok (()) , } } fn visit_ident_inner (& mut self , i : & mut Ident) { let s = i . to_string () ; if ! s . starts_with ("MACRO") || self . error . is_some () { return ; } match s . as_str () { "MACRO_FN_NAME" => * i = Ident :: new (self . fn_name , i . span ()) , "MACRO_FN_NAME_NORMALIZED" => * i = Ident :: new (& self . norm_name , i . span ()) , _ => { self . error = Some (syn :: Error :: new (i . span () , format ! ("unrecognized meta expression `{s}`") ,)) ; } } } }
};
}
