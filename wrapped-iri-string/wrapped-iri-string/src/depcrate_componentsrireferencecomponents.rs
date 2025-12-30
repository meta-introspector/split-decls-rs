// Generated macro for RiReferenceComponents (struct)
macro_rules! Depcrate_componentsRiReferenceComponents {
() => {
// Module: crate::components
// Provides: {"RiReferenceComponents"}
// Dependencies: {}
# [doc = " Components of an IRI reference."] # [doc = ""] # [doc = " See <https://tools.ietf.org/html/rfc3986#section-5.2.2>."] # [derive (Debug , Clone , Copy)] pub (crate) struct RiReferenceComponents < 'a , S : Spec > { # [doc = " Original complete string."] pub (crate) iri : & 'a RiReferenceStr < S > , # [doc = " Positions to split the IRI into components."] pub (crate) splitter : Splitter , }
};
}
