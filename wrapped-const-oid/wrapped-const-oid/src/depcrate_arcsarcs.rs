// Generated macro for Arcs (struct)
macro_rules! Depcrate_arcsArcs {
() => {
// Module: crate::arcs
// Provides: {"Arcs"}
// Dependencies: {}
# [doc = " [`Iterator`] over [`Arc`] values (a.k.a. nodes) in an [`ObjectIdentifier`]."] # [doc = ""] # [doc = " This iterates over all arcs in an OID, including the root."] pub struct Arcs < 'a > { # [doc = " OID bytes we're iterating over."] bytes : & 'a [u8] , # [doc = " Current position within the serialized BER bytes of this OID."] cursor : Option < usize > , }
};
}
