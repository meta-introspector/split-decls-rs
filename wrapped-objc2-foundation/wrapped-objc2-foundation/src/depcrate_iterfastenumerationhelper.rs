// Generated macro for FastEnumerationHelper (trait)
macro_rules! Depcrate_iterFastEnumerationHelper {
() => {
// Module: crate::iter
// Provides: {"FastEnumerationHelper"}
// Dependencies: {}
# [doc = " Internal helper trait to figure out the output type of something that"] # [doc = " implements `NSFastEnumeration`."] # [doc = ""] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The associated `Item` type must be the type that is used in Objective-C's"] # [doc = " `for (type elem in collection) { stmts; }` enumeration, and the collection"] # [doc = " itself must not contain any lifetime parameter."] pub (crate) unsafe trait FastEnumerationHelper : Message + NSFastEnumeration { type Item : Message ; fn maybe_len (& self) -> Option < usize > ; }
};
}
