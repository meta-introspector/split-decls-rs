// Generated macro for impl_68 (impl)
macro_rules! Depcrate_componentsimpl_68 {
() => {
// Module: crate::components
// Provides: {"impl_68"}
// Dependencies: {}
impl < 'a , S : Spec > RiReferenceComponents < 'a , S > { # [doc = " Returns five major components: scheme, authority, path, query, and fragment."] # [inline] # [must_use] pub (crate) fn to_major (self ,) -> (Option < & 'a str > , Option < & 'a str > , & 'a str , Option < & 'a str > , Option < & 'a str > ,) { self . splitter . split_into_major (self . iri . as_str ()) } # [doc = " Returns the IRI reference."] # [inline] # [must_use] pub (crate) fn iri (& self) -> & 'a RiReferenceStr < S > { self . iri } # [doc = " Returns the scheme as a string."] # [inline] # [must_use] pub (crate) fn scheme_str (& self) -> Option < & str > { self . splitter . scheme_str (self . iri . as_str ()) } # [doc = " Returns the authority as a string."] # [inline] # [must_use] pub (crate) fn authority_str (& self) -> Option < & str > { self . splitter . authority_str (self . iri . as_str ()) } # [doc = " Returns the path as a string."] # [inline] # [must_use] pub (crate) fn path_str (& self) -> & str { self . splitter . path_str (self . iri . as_str ()) } # [doc = " Returns the query as a string."] # [inline] # [must_use] pub (crate) fn query_str (& self) -> Option < & str > { self . splitter . query_str (self . iri . as_str ()) } }
};
}
