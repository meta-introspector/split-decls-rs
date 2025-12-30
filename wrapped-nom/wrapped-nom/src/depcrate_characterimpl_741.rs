// Generated macro for impl_741 (impl)
macro_rules! Depcrate_characterimpl_741 {
() => {
// Module: crate::character
// Provides: {"impl_741"}
// Dependencies: {}
impl < I , Error : ParseError < I > , F , MakeError > Parser < I > for Satisfy < F , MakeError > where I : Input , < I as Input > :: Item : AsChar , F : Fn (char) -> bool , MakeError : Fn (I) -> Error , { type Output = char ; type Error = Error ; # [inline (always)] fn process < OM : crate :: OutputMode > (& mut self , i : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { match (i) . iter_elements () . next () . map (| t | { let c = t . as_char () ; let b = (self . predicate) (c) ; (c , b) }) { None => { if OM :: Incomplete :: is_streaming () { Err (Err :: Incomplete (Needed :: Unknown)) } else { Err (Err :: Error (OM :: Error :: bind (| | (self . make_error) (i)))) } } Some ((_ , false)) => Err (Err :: Error (OM :: Error :: bind (| | (self . make_error) (i)))) , Some ((c , true)) => Ok ((i . take_from (c . len ()) , OM :: Output :: bind (| | c . as_char ()))) , } } }
};
}
