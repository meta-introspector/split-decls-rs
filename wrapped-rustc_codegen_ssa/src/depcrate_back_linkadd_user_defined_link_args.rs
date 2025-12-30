// Generated macro for add_user_defined_link_args (function)
macro_rules! Depcrate_back_linkadd_user_defined_link_args {
() => {
// Module: crate::back::link
// Provides: {"add_user_defined_link_args"}
// Dependencies: {}
# [doc = " Add arbitrary \"user defined\" args defined from command line."] # [doc = " FIXME: Determine where exactly these args need to be inserted."] fn add_user_defined_link_args (cmd : & mut dyn Linker , sess : & Session) { cmd . verbatim_args (& sess . opts . cg . link_args) ; }
};
}
