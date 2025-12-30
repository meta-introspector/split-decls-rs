// Generated macro for impl_738 (impl)
macro_rules! Depcrate_characterimpl_738 {
() => {
// Module: crate::character
// Provides: {"impl_738"}
// Dependencies: {}
impl < I , Error : ParseError < I > > Parser < I > for Char < Error > where I : Input , < I as Input > :: Item : AsChar , { type Output = char ; type Error = Error ; # [inline (always)] fn process < OM : crate :: OutputMode > (& mut self , i : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { match (i) . iter_elements () . next () . map (| t | { let b = t . as_char () == self . c ; (& self . c , b) }) { None => { if OM :: Incomplete :: is_streaming () { Err (Err :: Incomplete (Needed :: new (self . c . len () - i . input_len ()))) } else { Err (Err :: Error (OM :: Error :: bind (| | Error :: from_char (i , self . c)))) } } Some ((_ , false)) => Err (Err :: Error (OM :: Error :: bind (| | Error :: from_char (i , self . c)))) , Some ((c , true)) => Ok ((i . take_from (c . len ()) , OM :: Output :: bind (| | c . as_char ()))) , } } }
};
}
