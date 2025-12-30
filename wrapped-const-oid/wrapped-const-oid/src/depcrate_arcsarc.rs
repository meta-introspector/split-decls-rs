// Generated macro for Arc (type)
macro_rules! Depcrate_arcsArc {
() => {
// Module: crate::arcs
// Provides: {"Arc"}
// Dependencies: {}
# [doc = " Type alias used to represent an \"arc\", i.e. integer identifier value, where an OID comprises a"] # [doc = " sequence of arcs."] # [doc = ""] # [doc = " X.660 does not define a maximum size of an arc. We instead follow Mozilla* conventions for"] # [doc = " maximum values of an arc, with a maximum value of 2^32-1 (4294967295), a.k.a. [`u32::MAX`]"] # [doc = " with [`Arc`] being a type alias for [`u32`]."] # [doc = ""] # [doc = " Note that this means we deliberately do *NOT* support UUIDs used as OIDs."] # [doc = ""] # [doc = " *NOTE: please see this study for a survey of how various OID libraries handle maximum arcs:"] # [doc = " <https://misc.daniel-marschall.de/asn.1/oid_facts.html>"] pub type Arc = u32 ;
};
}
