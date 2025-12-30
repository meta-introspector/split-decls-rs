// Generated macro for argall_impl (macro)
macro_rules! Depcrate_arg_msgargargall_impl {
() => {
// Module: crate::arg::msgarg
// Provides: {"argall_impl"}
// Dependencies: {}
macro_rules ! argall_impl { ($ ($ n : ident $ t : ident $ s : ty ,) +) => { impl <$ ($ t : Arg) ,*> ArgAll for ($ ($ t ,) *) { type strs = ($ (&'static $ s ,) *) ; fn strs_sig < Q : FnMut (&'static str , Signature <'static >) > (z : Self :: strs , mut q : Q) { let ($ ($ n ,) *) = z ; $ (q ($ n , $ t :: signature ()) ;) * } } impl <$ ($ t : Append) ,*> AppendAll for ($ ($ t ,) *) { fn append (& self , ia : & mut IterAppend) { let ($ ($ n ,) *) = self ; $ (ia . append ($ n) ;) * } } impl <$ ($ t : Arg + for <'z > Get <'z >) ,*> ReadAll for ($ ($ t ,) *) { fn read (ii : & mut Iter) -> Result < Self , TypeMismatchError > { $ (let $ n = ii . read () ?;) * Ok (($ ($ n ,) *)) } } } }
};
}
