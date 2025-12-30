// Generated macro for Debug2Format (struct)
macro_rules! Depcrate_impls_adapterDebug2Format {
() => {
// Module: crate::impls::adapter
// Provides: {"Debug2Format"}
// Dependencies: {}
# [doc = " An \"adapter\" type to feed `Debug` values into defmt macros, which expect `defmt::Format` values."] # [doc = ""] # [doc = " This adapter disables compression and uses the `core::fmt` code on-device! You should prefer"] # [doc = " `defmt::Format` over `Debug` whenever possible."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[derive(Debug)]"] # [doc = " # struct ExpensiveThing();"] # [doc = " # let expensive_thing = ExpensiveThing();"] # [doc = " #"] # [doc = " defmt::info!(\"{:?}\", defmt::Debug2Format(&expensive_thing));"] # [doc = " //                                        ˆˆˆˆˆˆˆˆˆˆˆˆˆˆˆ"] # [doc = " //                                        must `#[derive(Debug)]`"] # [doc = " ```"] # [doc = ""] # [doc = " Note that any provided defmt display hints will be ignored"] # [doc = " because this always uses `{:?}` to format the contained value."] pub struct Debug2Format < 'a , T : fmt :: Debug + ? Sized > (pub & 'a T) ;
};
}
