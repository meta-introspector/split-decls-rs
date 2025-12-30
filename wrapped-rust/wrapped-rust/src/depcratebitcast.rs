// Generated macro for bitcast (function)
macro_rules! Depcratebitcast {
() => {
// Module: crate
// Provides: {"bitcast"}
// Dependencies: {}
fn bitcast (casts : & [Bitcast] , operands : & [String] , results : & mut Vec < String >) { for (cast , operand) in casts . iter () . zip (operands) { results . push (perform_cast (operand , cast)) ; } }
};
}
