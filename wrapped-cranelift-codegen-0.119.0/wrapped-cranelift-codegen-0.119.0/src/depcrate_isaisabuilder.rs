// Generated macro for IsaBuilder (struct)
macro_rules! Depcrate_isaIsaBuilder {
() => {
// Module: crate::isa
// Provides: {"IsaBuilder"}
// Dependencies: {}
# [doc = " Builder for a `TargetIsa`."] # [doc = " Modify the ISA-specific settings before creating the `TargetIsa` trait object with `finish`."] # [derive (Clone)] pub struct IsaBuilder < T > { triple : Triple , setup : settings :: Builder , constructor : fn (Triple , settings :: Flags , & settings :: Builder) -> T , }
};
}
