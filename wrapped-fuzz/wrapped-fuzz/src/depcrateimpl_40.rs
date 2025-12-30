// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl Scenario < '_ > { # [doc = " Run the provided scenario, asserting for correct behavior"] pub fn run (self) { if let Some ((compact , mut control)) = self . creation . create () { let mut compact = assert_not_option (compact) ; let max_num_actions = generate_rand_max_num_actions (self . seed) ; self . actions . into_iter () . take (max_num_actions) . for_each (| a | a . perform (& mut control , & mut compact)) ; assert_eq ! (compact , control) ; unsafe { assert_eq ! (compact . as_bytes_mut () , control . as_bytes_mut ()) } ; let debug_compact = format ! ("{:?}" , compact) ; let debug_std_str = format ! ("{:?}" , control) ; assert_eq ! (debug_compact , debug_std_str) ; # [allow (clippy :: useless_format)] let display_compact = format ! ("{}" , compact) ; # [allow (clippy :: useless_format)] let display_std_str = format ! ("{}" , control) ; assert_eq ! (display_compact , display_std_str) ; let compact = assert_not_option (compact) ; let compact_into_string = String :: from (compact) ; assert_eq ! (compact_into_string , control) ; } } }
};
}
