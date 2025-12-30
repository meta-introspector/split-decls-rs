// Generated macro for impl_825 (impl)
macro_rules! Depcrate_output_help_templateimpl_825 {
() => {
// Module: crate::output::help_template
// Provides: {"impl_825"}
// Dependencies: {}
impl < 'cmd , 'writer > AutoHelp < 'cmd , 'writer > { # [doc = " Create a new `HelpTemplate` instance."] pub (crate) fn new (writer : & 'writer mut StyledStr , cmd : & 'cmd Command , usage : & 'cmd Usage < 'cmd > , use_long : bool ,) -> Self { Self { template : HelpTemplate :: new (writer , cmd , usage , use_long) , } } pub (crate) fn write_help (& mut self) { let pos = self . template . cmd . get_positionals () . any (| arg | should_show_arg (self . template . use_long , arg)) ; let non_pos = self . template . cmd . get_non_positionals () . any (| arg | should_show_arg (self . template . use_long , arg)) ; let subcmds = self . template . cmd . has_visible_subcommands () ; let template = if non_pos || pos || subcmds { DEFAULT_TEMPLATE } else { DEFAULT_NO_ARGS_TEMPLATE } ; self . template . write_templated_help (template) ; } }
};
}
