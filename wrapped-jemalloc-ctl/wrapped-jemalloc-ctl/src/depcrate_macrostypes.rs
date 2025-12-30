// Generated macro for types (macro)
macro_rules! Depcrate_macrostypes {
() => {
// Module: crate::macros
// Provides: {"types"}
// Dependencies: {}
macro_rules ! types { ($ id : ident [str : $ byte_string : expr , $ mib : ty , $ name_to_mib : ident] | docs : $ (# [$ doc : meta]) * mib_docs : $ (# [$ doc_mib : meta]) *) => { paste :: paste ! { $ (# [$ doc]) * # [allow (non_camel_case_types)] pub struct $ id ; impl $ id { const NAME : &'static crate :: keys :: Name = { union U <'a > { bytes : &'a [u8] , name : &'a crate :: keys :: Name } unsafe { U { bytes : $ byte_string } . name } } ; # [doc = " Returns Management Information Base (MIB)"] # [doc = ""] # [doc = " This value can be used to access the key without doing string lookup."] pub fn mib () -> crate :: error :: Result < [<$ id _mib >] > { Ok ([<$ id _mib >] (Self :: NAME .$ name_to_mib () ?)) } # [doc = " Key [`crate::keys::Name`]."] pub fn name () -> &'static crate :: keys :: Name { Self :: NAME } } $ (# [$ doc_mib]) * # [repr (transparent)] # [derive (Copy , Clone)] # [allow (non_camel_case_types)] pub struct [<$ id _mib >] (pub crate :: keys ::$ mib) ; } } ; }
};
}
