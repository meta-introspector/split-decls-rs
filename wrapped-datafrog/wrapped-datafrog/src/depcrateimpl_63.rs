// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
impl Iteration { # [doc = " Create a new iterative context."] pub fn new () -> Self { Iteration { variables : Vec :: new () , } } # [doc = " Reports whether any of the monitored variables have changed since"] # [doc = " the most recent call."] pub fn changed (& mut self) -> bool { let mut result = false ; for variable in self . variables . iter_mut () { if variable . changed () { result = true ; } } result } # [doc = " Creates a new named variable associated with the iterative context."] pub fn variable < Tuple : Ord + 'static > (& mut self , name : & str) -> Variable < Tuple > { let variable = Variable :: new (name) ; self . variables . push (Box :: new (variable . clone ())) ; variable } # [doc = " Creates a new named variable associated with the iterative context."] # [doc = ""] # [doc = " This variable will not be maintained distinctly, and may advertise tuples as"] # [doc = " recent multiple times (perhaps unboundedly many times)."] pub fn variable_indistinct < Tuple : Ord + 'static > (& mut self , name : & str) -> Variable < Tuple > { let mut variable = Variable :: new (name) ; variable . distinct = false ; self . variables . push (Box :: new (variable . clone ())) ; variable } }
};
}
