// Generated macro for private (module)
macro_rules! Depcrate_valueprivate {
() => {
// Module: crate::value
// Provides: {"private"}
// Dependencies: {}
mod private { use crate :: stream :: Stream ; pub trait DispatchValue { fn dispatch_stream < 'sval > (& 'sval self , stream : & mut dyn Stream < 'sval >) -> sval :: Result ; fn dispatch_tag (& self) -> Option < sval :: Tag > ; fn dispatch_to_bool (& self) -> Option < bool > ; fn dispatch_to_f32 (& self) -> Option < f32 > ; fn dispatch_to_f64 (& self) -> Option < f64 > ; fn dispatch_to_i8 (& self) -> Option < i8 > ; fn dispatch_to_i16 (& self) -> Option < i16 > ; fn dispatch_to_i32 (& self) -> Option < i32 > ; fn dispatch_to_i64 (& self) -> Option < i64 > ; fn dispatch_to_i128 (& self) -> Option < i128 > ; fn dispatch_to_u8 (& self) -> Option < u8 > ; fn dispatch_to_u16 (& self) -> Option < u16 > ; fn dispatch_to_u32 (& self) -> Option < u32 > ; fn dispatch_to_u64 (& self) -> Option < u64 > ; fn dispatch_to_u128 (& self) -> Option < u128 > ; fn dispatch_to_text (& self) -> Option < & str > ; fn dispatch_to_binary (& self) -> Option < & [u8] > ; } pub trait EraseValue { fn erase_value (& self) -> crate :: private :: Erased < & dyn DispatchValue > ; } }
};
}
