// Generated macro for FlagsOrIsa (struct)
macro_rules! Depcrate_settingsFlagsOrIsa {
() => {
// Module: crate::settings
// Provides: {"FlagsOrIsa"}
// Dependencies: {}
# [doc = " Wrapper containing flags and optionally a `TargetIsa` trait object."] # [doc = ""] # [doc = " A few passes need to access the flags but only optionally a target ISA. The `FlagsOrIsa`"] # [doc = " wrapper can be used to pass either, and extract the flags so they are always accessible."] # [derive (Clone , Copy)] pub struct FlagsOrIsa < 'a > { # [doc = " Flags are always present."] pub flags : & 'a Flags , # [doc = " The ISA may not be present."] pub isa : Option < & 'a dyn TargetIsa > , }
};
}
