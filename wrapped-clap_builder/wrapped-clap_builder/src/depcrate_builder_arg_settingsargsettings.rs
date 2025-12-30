// Generated macro for ArgSettings (enum)
macro_rules! Depcrate_builder_arg_settingsArgSettings {
() => {
// Module: crate::builder::arg_settings
// Provides: {"ArgSettings"}
// Dependencies: {}
# [doc = " Various settings that apply to arguments and may be set, unset, and checked via getter/setter"] # [doc = " methods [`Arg::setting`], [`Arg::unset_setting`], and [`Arg::is_set`]. This is what the"] # [doc = " [`Arg`] methods which accept a `bool` use internally."] # [doc = ""] # [doc = " [`Arg`]: crate::Arg"] # [doc = " [`Arg::setting`]: crate::Arg::setting()"] # [doc = " [`Arg::unset_setting`]: crate::Arg::unset_setting()"] # [doc = " [`Arg::is_set`]: crate::Arg::is_set()"] # [derive (Debug , PartialEq , Copy , Clone)] # [repr (u8)] pub (crate) enum ArgSettings { Required , Global , Hidden , NextLineHelp , HidePossibleValues , AllowHyphenValues , AllowNegativeNumbers , RequireEquals , Last , TrailingVarArg , HideDefaultValue , IgnoreCase , # [cfg (feature = "env")] HideEnv , # [cfg (feature = "env")] HideEnvValues , HiddenShortHelp , HiddenLongHelp , Exclusive , }
};
}
