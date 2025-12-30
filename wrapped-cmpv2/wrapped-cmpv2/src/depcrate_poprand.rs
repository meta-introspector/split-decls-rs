// Generated macro for Rand (struct)
macro_rules! Depcrate_popRand {
() => {
// Module: crate::pop
// Provides: {"Rand"}
// Dependencies: {}
# [doc = " The `Rand` type is defined as a comment in the `Challenge` definition in"] # [doc = " [RFC 4210 Section 5.2.8.3]."] # [doc = ""] # [doc = " ```text"] # [doc = "    Rand ::= SEQUENCE {"] # [doc = "        int      INTEGER,"] # [doc = "        sender   GeneralName"] # [doc = "   }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.2.8.3]: https://www.rfc-editor.org/rfc/rfc4210#section-5.2.8.3"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct Rand < 'a > { pub integer : UintRef < 'a > , pub sender : GeneralName , }
};
}
