// Generated macro for htmldocck (function)
macro_rules! Depcrate_external_deps_htmldocckhtmldocck {
() => {
// Module: crate::external_deps::htmldocck
// Provides: {"htmldocck"}
// Dependencies: {}
# [doc = " `htmldocck` is a python script which is used for rustdoc test suites, it is assumed to be"] # [doc = " available at `$SOURCE_ROOT/src/etc/htmldocck.py`."] # [track_caller] # [must_use] pub fn htmldocck () -> Command { let mut python = python_command () ; python . arg (source_root () . join ("src/etc/htmldocck.py")) ; python }
};
}
