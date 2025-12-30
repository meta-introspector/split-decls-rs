// Generated macro for clif_intcast (function)
macro_rules! Depcrate_castclif_intcast {
() => {
// Module: crate::cast
// Provides: {"clif_intcast"}
// Dependencies: {}
pub (crate) fn clif_intcast (fx : & mut FunctionCx < '_ , '_ , '_ > , val : Value , to : Type , signed : bool ,) -> Value { let from = fx . bcx . func . dfg . value_type (val) ; match (from , to) { (_ , _) if from == to => val , (_ , _) if to . wider_or_equal (from) => { if signed { fx . bcx . ins () . sextend (to , val) } else { fx . bcx . ins () . uextend (to , val) } } (_ , _) => fx . bcx . ins () . ireduce (to , val) , } }
};
}
