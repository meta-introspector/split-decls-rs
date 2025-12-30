// Generated macro for difference_or_zero (function)
macro_rules! Depcrate_builderdifference_or_zero {
() => {
// Module: crate::builder
// Provides: {"difference_or_zero"}
// Dependencies: {}
fn difference_or_zero < 'gcc > (loc : Option < Location < 'gcc > > , a : RValue < 'gcc > , b : RValue < 'gcc > , context : & 'gcc Context < 'gcc > ,) -> RValue < 'gcc > { let difference = a - b ; let masks = context . new_comparison (loc , ComparisonOp :: GreaterThanEquals , b , a) ; let a_type = a . get_type () ; let masks = if masks . get_type () != a_type { context . new_bitcast (loc , masks , a_type) } else { masks } ; difference & masks }
};
}
