// Generated macro for impl_replace (macro)
macro_rules! Depcrate_mask_passwordimpl_replace {
() => {
// Module: crate::mask_password
// Provides: {"impl_replace"}
// Dependencies: {}
# [doc = " Implements traits for `PasswordReplaced`."] macro_rules ! impl_replace { ($ borrowed : ident , $ owned : ident) => { impl < S : Spec , D : fmt :: Display > fmt :: Display for PasswordReplaced <'_ , $ borrowed < S >, D > { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { match & self . password { Some ((pw_range , alt)) => { write_with_masked_password (f , self . iri_ref . as_str () , pw_range . clone () , alt) } None => self . iri_ref . fmt (f) , } } } impl < S : Spec , D : fmt :: Display > fmt :: Debug for PasswordReplaced <'_ , $ borrowed < S >, D > { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { f . write_char ('<') ?; fmt :: Display :: fmt (self , f) ?; f . write_char ('>') } } } ; }
};
}
