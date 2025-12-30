// Generated macro for InputValueError (struct)
macro_rules! Depcrate_errorInputValueError {
() => {
// Module: crate::error
// Provides: {"InputValueError"}
// Dependencies: {}
# [doc = " An error parsing an input value."] # [doc = ""] # [doc = " This type is generic over T as it uses T's type name when converting to a"] # [doc = " regular error."] # [derive (Debug)] pub struct InputValueError < T > { message : String , extensions : Option < ErrorExtensionValues > , phantom : PhantomData < T > , }
};
}
