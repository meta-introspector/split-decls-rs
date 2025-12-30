// Generated macro for build_decls (function)
macro_rules! Depcrate_buildbuild_decls {
() => {
// Module: crate::build
// Provides: {"build_decls"}
// Dependencies: {}
fn build_decls (crate_ : & Path , input : & ForeignType) -> TokenStream { let attrs = & input . attrs ; let vis = & input . visibility ; let name = & input . name ; let generics = & input . generics ; let ctype = & input . ctype ; let phantom_data = input . phantom_data . as_ref () . map (| d | quote ! (, # crate_ :: export :: PhantomData <# d >)) ; let ref_name = ref_name (input) ; let ref_docs = format ! ("A borrowed reference to a [`{name}`](struct.{name}.html)." , name = name) ; quote ! { # (# attrs) * # [repr (transparent)] # vis struct # name # generics (# crate_ :: export :: NonNull <# ctype > # phantom_data) ; # [doc = # ref_docs] # vis struct # ref_name # generics (# crate_ :: Opaque # phantom_data) ; } }
};
}
