// Generated macro for verify_constructor_return (function)
macro_rules! Depcrate_witverify_constructor_return {
() => {
// Module: crate::wit
// Provides: {"verify_constructor_return"}
// Dependencies: {}
# [doc = " Verifies exported constructor return value is not a JS primitive type"] fn verify_constructor_return (class : & str , ret : & Descriptor) -> Result < () , Error > { match ret { Descriptor :: I8 | Descriptor :: U8 | Descriptor :: ClampedU8 | Descriptor :: I16 | Descriptor :: U16 | Descriptor :: I32 | Descriptor :: U32 | Descriptor :: F32 | Descriptor :: F64 | Descriptor :: I64 | Descriptor :: U64 | Descriptor :: Boolean | Descriptor :: Char | Descriptor :: CachedString | Descriptor :: String | Descriptor :: Option (_) | Descriptor :: Enum { .. } | Descriptor :: Unit => { bail ! ("The constructor for class `{class}` tries to return a JS primitive type, which would cause the return value to be ignored. Use a builder instead (remove the `constructor` attribute).") ; } Descriptor :: Result (ref d) | Descriptor :: Ref (ref d) | Descriptor :: RefMut (ref d) => { verify_constructor_return (class , d) } _ => Ok (()) , } }
};
}
