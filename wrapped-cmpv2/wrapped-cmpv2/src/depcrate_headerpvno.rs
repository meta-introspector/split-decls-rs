// Generated macro for Pvno (enum)
macro_rules! Depcrate_headerPvno {
() => {
// Module: crate::header
// Provides: {"Pvno"}
// Dependencies: {}
# [doc = " The `PKIHeader` type defined in [RFC 4210 Section 5.1.1] features an inline INTEGER definition"] # [doc = " that is implemented as the Pvno enum."] # [doc = ""] # [doc = " ```text"] # [doc = "     pvno                INTEGER     { cmp1999(1), cmp2000(2) },"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.1.1]: https://datatracker.ietf.org/doc/html/rfc4210#section-5.1.1"] # [derive (Clone , Debug , Copy , PartialEq , Eq , Enumerated , Ord , PartialOrd)] # [asn1 (type = "INTEGER")] # [repr (u8)] # [allow (missing_docs)] pub enum Pvno { Cmp1999 = 1 , Cmp2000 = 2 , }
};
}
