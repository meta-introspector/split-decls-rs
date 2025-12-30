// Generated macro for is_implicit_trivial (function)
macro_rules! Depcrateis_implicit_trivial {
() => {
// Module: crate
// Provides: {"is_implicit_trivial"}
// Dependencies: {}
# [allow (clippy :: unnecessary_wraps)] fn is_implicit_trivial (field : & Field) -> Result < bool > { match & field . ty { Type :: Tuple (ty) => Ok (ty . elems . is_empty ()) , Type :: Path (ty) => { let ident = & ty . path . segments . last () . unwrap () . ident ; Ok (ident == "PhantomData" || ident == "PhantomPinned") } _ => Ok (false) , } }
};
}
