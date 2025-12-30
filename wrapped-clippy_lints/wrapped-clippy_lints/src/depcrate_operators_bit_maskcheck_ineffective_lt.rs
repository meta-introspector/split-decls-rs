// Generated macro for check_ineffective_lt (function)
macro_rules! Depcrate_operators_bit_maskcheck_ineffective_lt {
() => {
// Module: crate::operators::bit_mask
// Provides: {"check_ineffective_lt"}
// Dependencies: {}
fn check_ineffective_lt (cx : & LateContext < '_ > , span : Span , m : u128 , c : u128 , op : & str) { if c . is_power_of_two () && m < c { span_lint (cx , INEFFECTIVE_BIT_MASK , span , format ! ("ineffective bit mask: `x {op} {m}` compared to `{c}`, is the same as x compared directly") ,) ; } }
};
}
