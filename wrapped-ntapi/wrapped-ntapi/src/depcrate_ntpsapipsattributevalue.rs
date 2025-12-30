// Generated macro for PsAttributeValue (function)
macro_rules! Depcrate_ntpsapiPsAttributeValue {
() => {
// Module: crate::ntpsapi
// Provides: {"PsAttributeValue"}
// Dependencies: {}
# [inline] pub const fn PsAttributeValue (mut Number : PS_ATTRIBUTE_NUM , Thread : bool , Input : bool , Additive : bool ,) -> ULONG_PTR { Number &= PS_ATTRIBUTE_NUMBER_MASK ; if Thread { Number |= PS_ATTRIBUTE_THREAD ; } if Input { Number |= PS_ATTRIBUTE_INPUT ; } if Additive { Number |= PS_ATTRIBUTE_ADDITIVE ; } Number as _ }
};
}
