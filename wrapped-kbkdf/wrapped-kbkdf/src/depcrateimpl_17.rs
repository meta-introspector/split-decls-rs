// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'k , 'l , 'c > ParamsBuilder < 'k , 'l , 'c > { # [doc = " Return the built [`Params`]"] pub fn build (self) -> Params < 'k , 'l , 'c > { self . 0 } # [doc = " Set the label for the parameters"] pub fn with_label (mut self , label : & 'l [u8]) -> Self { self . 0 . label = label ; self } # [doc = " Set the context for the parameters"] pub fn with_context (mut self , context : & 'c [u8]) -> Self { self . 0 . context = context ; self } # [doc = " During the iterations, append the length of the Prf"] pub fn use_l (mut self , use_l : bool) -> Self { self . 0 . use_l = use_l ; self } # [doc = " During the iterations, separate the label from the context with a NULL byte"] pub fn use_separator (mut self , use_separator : bool) -> Self { self . 0 . use_separator = use_separator ; self } # [doc = " During the iterations, update the Prf with the iteration counter"] pub fn use_counter (mut self , use_counter : bool) -> Self { self . 0 . use_counter = use_counter ; self } }
};
}
