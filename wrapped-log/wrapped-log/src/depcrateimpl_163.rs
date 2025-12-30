// Generated macro for impl_163 (impl)
macro_rules! Depcrateimpl_163 {
() => {
// Module: crate
// Provides: {"impl_163"}
// Dependencies: {}
impl < 'a > Metadata < 'a > { # [doc = " Returns a new builder."] # [inline] pub fn builder () -> MetadataBuilder < 'a > { MetadataBuilder :: new () } # [doc = " The verbosity level of the message."] # [inline] pub fn level (& self) -> Level { self . level } # [doc = " The name of the target of the directive."] # [inline] pub fn target (& self) -> & 'a str { self . target } }
};
}
