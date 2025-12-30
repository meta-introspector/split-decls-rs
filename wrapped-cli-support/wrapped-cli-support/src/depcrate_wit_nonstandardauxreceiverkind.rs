// Generated macro for AuxReceiverKind (enum)
macro_rules! Depcrate_wit_nonstandardAuxReceiverKind {
() => {
// Module: crate::wit::nonstandard
// Provides: {"AuxReceiverKind"}
// Dependencies: {}
# [doc = " The 'receiver' of a method; in other words, the type that the method is called on."] # [doc = ""] # [doc = " This is `None` if the method is static, or `Borrowed` or `Owned` if the"] # [doc = " method takes `&[mut] self` or `self` respectively."] # [derive (Debug , Clone , Copy)] pub enum AuxReceiverKind { None , Borrowed , Owned , }
};
}
