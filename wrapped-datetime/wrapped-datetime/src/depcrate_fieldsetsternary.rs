// Generated macro for ternary (macro)
macro_rules! Depcrate_fieldsetsternary {
() => {
// Module: crate::fieldsets
// Provides: {"ternary"}
// Dependencies: {}
macro_rules ! ternary { ($ present : expr , $ missing : expr , yes) => { $ present } ; ($ present : expr , $ missing : expr , $ any : literal) => { $ present } ; ($ present : expr , $ missing : expr ,) => { $ missing } ; }
};
}
