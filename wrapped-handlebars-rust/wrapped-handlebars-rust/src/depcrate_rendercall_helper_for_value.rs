// Generated macro for call_helper_for_value (function)
macro_rules! Depcrate_rendercall_helper_for_value {
() => {
// Module: crate::render
// Provides: {"call_helper_for_value"}
// Dependencies: {}
# [inline] fn call_helper_for_value < 'reg : 'rc , 'rc > (hd : & dyn HelperDef , ht : & Helper < 'rc > , r : & 'reg Registry < 'reg > , ctx : & 'rc Context , rc : & mut RenderContext < 'reg , 'rc > ,) -> Result < PathAndJson < 'rc > , RenderError > { match hd . call_inner (ht , r , ctx , rc) { Ok (result) => Ok (PathAndJson :: new (None , result)) , Err (e) => { if e . is_unimplemented () { let mut so = StringOutput :: new () ; let disable_escape = rc . is_disable_escape () ; rc . set_disable_escape (true) ; hd . call (ht , r , ctx , rc , & mut so) ? ; rc . set_disable_escape (disable_escape) ; let string = so . into_string () . map_err (RenderError :: from) ? ; Ok (PathAndJson :: new (None , ScopedJson :: Derived (Json :: String (string)) ,)) } else { Err (e) } } } }
};
}
