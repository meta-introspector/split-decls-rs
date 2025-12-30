// Generated macro for impl_25 (impl)
macro_rules! Depcrate_covfunimpl_25 {
() => {
// Module: crate::covfun
// Provides: {"impl_25"}
// Dependencies: {}
impl ExpressionResolver { fn new () -> Self { Self { operands : Vec :: new () } } fn push_operands (& mut self , lhs : CovTerm , rhs : CovTerm) { self . operands . push ((lhs , rhs)) ; } fn format_term (& self , term : CovTerm) -> String { let mut output = String :: new () ; self . write_term (& mut output , term) ; output } fn write_term (& self , output : & mut String , term : CovTerm) { match term { CovTerm :: Zero => output . push_str ("Zero") , CovTerm :: Counter (id) => write ! (output , "c{id}") . unwrap () , CovTerm :: Expression (id , op) => { let (lhs , rhs) = self . operands [id as usize] ; let op = match op { Op :: Sub => "-" , Op :: Add => "+" , } ; output . push ('(') ; self . write_term (output , lhs) ; write ! (output , " {op} ") . unwrap () ; self . write_term (output , rhs) ; output . push (')') ; } } } }
};
}
