// Generated macro for AppSettings (enum)
macro_rules! Depcrate_builder_app_settingsAppSettings {
() => {
// Module: crate::builder::app_settings
// Provides: {"AppSettings"}
// Dependencies: {}
# [doc = " Application level settings, which affect how [`Command`] operates"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **NOTE:** When these settings are used, they apply only to current command, and are *not*"] # [doc = " propagated down or up through child or parent subcommands"] # [doc = ""] # [doc = " </div>"] # [doc = ""] # [doc = " [`Command`]: crate::Command"] # [derive (Debug , PartialEq , Copy , Clone)] # [repr (u8)] pub (crate) enum AppSettings { IgnoreErrors , AllowHyphenValues , AllowNegativeNumbers , AllArgsOverrideSelf , AllowMissingPositional , TrailingVarArg , DontDelimitTrailingValues , InferLongArgs , InferSubcommands , SubcommandRequired , AllowExternalSubcommands , Multicall , SubcommandsNegateReqs , ArgsNegateSubcommands , SubcommandPrecedenceOverArg , FlattenHelp , ArgRequiredElseHelp , NextLineHelp , DisableColoredHelp , DisableHelpFlag , DisableHelpSubcommand , DisableVersionFlag , PropagateVersion , Hidden , HidePossibleValues , HelpExpected , NoBinaryName , # [allow (dead_code)] ColorAuto , ColorAlways , ColorNever , Built , BinNameBuilt , }
};
}
