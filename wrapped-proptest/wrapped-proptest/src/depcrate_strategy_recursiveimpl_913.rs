// Generated macro for impl_913 (impl)
macro_rules! Depcrate_strategy_recursiveimpl_913 {
() => {
// Module: crate::strategy::recursive
// Provides: {"impl_913"}
// Dependencies: {}
impl < T : fmt :: Debug + 'static , R : Strategy < Value = T > + 'static , F : Fn (BoxedStrategy < T >) -> R , > Strategy for Recursive < T , F > { type Tree = Box < dyn ValueTree < Value = T > > ; type Value = T ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { let mut branch_probabilities = Vec :: new () ; let mut k2 = u64 :: from (self . expected_branch_size) * 2 ; for _ in 0 .. self . depth { branch_probabilities . push (f64 :: from (self . desired_size) / k2 as f64) ; k2 = k2 . saturating_mul (u64 :: from (self . expected_branch_size) * 2) ; } let mut strat = self . base . clone () ; while let Some (branch_probability) = branch_probabilities . pop () { let recursed = (self . recurse) (strat . clone ()) ; let recursive_choice = recursed . boxed () ; let non_recursive_choice = strat ; let branch_probability = branch_probability . min (0.9) ; let (weight_branch , weight_leaf) = float_to_weight (branch_probability) ; let branch = prop_oneof ! [weight_leaf => non_recursive_choice , weight_branch => recursive_choice ,] ; strat = branch . boxed () ; } strat . new_tree (runner) } }
};
}
