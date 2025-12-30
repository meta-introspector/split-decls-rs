// Generated macro for link_macro (macro)
macro_rules! Depcrate_windows_linklink_macro {
() => {
// Module: crate::windows_link
// Provides: {"link_macro"}
// Dependencies: {}
macro_rules ! link_macro { ($ library : literal $ abi : literal $ ($ link_name : literal) ? $ (# [$ doc : meta]) ? fn $ ($ function : tt) *) => (# [link (name = "kernel32")] extern $ abi { $ (# [link_name =$ link_name]) ? pub fn $ ($ function) *; }) }
};
}
