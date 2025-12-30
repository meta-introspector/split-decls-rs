// Generated macro for impl_855 (impl)
macro_rules! Depcrate_output_usageimpl_855 {
() => {
// Module: crate::output::usage
// Provides: {"impl_855"}
// Dependencies: {}
impl < 'cmd > Usage < 'cmd > { pub (crate) fn new (cmd : & 'cmd Command) -> Self { Usage { cmd , styles : cmd . get_styles () , required : None , } } pub (crate) fn required (mut self , required : & 'cmd ChildGraph < Id >) -> Self { self . required = Some (required) ; self } pub (crate) fn create_usage_with_title (& self , used : & [Id]) -> Option < StyledStr > { debug ! ("Usage::create_usage_with_title") ; use std :: fmt :: Write as _ ; let mut styled = StyledStr :: new () ; let _ = write ! (styled , "{}Usage:{} " , self . styles . get_usage () . render () , self . styles . get_usage () . render_reset ()) ; if self . write_usage_no_title (& mut styled , used) { styled . trim_end () ; } else { return None ; } debug ! ("Usage::create_usage_with_title: usage={styled}") ; Some (styled) } pub (crate) fn create_usage_no_title (& self , used : & [Id]) -> Option < StyledStr > { debug ! ("Usage::create_usage_no_title") ; let mut styled = StyledStr :: new () ; if self . write_usage_no_title (& mut styled , used) { styled . trim_end () ; debug ! ("Usage::create_usage_no_title: usage={styled}") ; Some (styled) } else { None } } fn write_usage_no_title (& self , styled : & mut StyledStr , used : & [Id]) -> bool { debug ! ("Usage::create_usage_no_title") ; if let Some (u) = self . cmd . get_override_usage () { styled . push_styled (u) ; true } else { # [cfg (feature = "usage")] { if used . is_empty () { self . write_help_usage (styled) ; } else { self . write_smart_usage (styled , used) ; } true } # [cfg (not (feature = "usage"))] { false } } } }
};
}
