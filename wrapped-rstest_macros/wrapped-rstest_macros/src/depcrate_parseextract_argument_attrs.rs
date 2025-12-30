// Generated macro for extract_argument_attrs (function)
macro_rules! Depcrate_parseextract_argument_attrs {
() => {
// Module: crate::parse
// Provides: {"extract_argument_attrs"}
// Dependencies: {}
pub (crate) fn extract_argument_attrs < 'a , B : 'a + std :: fmt :: Debug > (node : & mut FnArg , is_valid_attr : fn (& syn :: Attribute) -> bool , build : impl Fn (syn :: Attribute) -> syn :: Result < B > + 'a ,) -> Box < dyn Iterator < Item = syn :: Result < B > > + 'a > { let name = node . maybe_ident () . cloned () ; if name . is_none () { return Box :: new (std :: iter :: empty ()) ; } if let FnArg :: Typed (ref mut arg) = node { let attrs = std :: mem :: take (& mut arg . attrs) ; let (extracted , remain) : (Vec < _ > , Vec < _ >) = attrs . into_iter () . partition (is_valid_attr) ; arg . attrs = remain ; Box :: new (extracted . into_iter () . map (build)) } else { Box :: new (std :: iter :: empty ()) } }
};
}
