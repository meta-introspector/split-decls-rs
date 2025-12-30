// Generated macro for impl_93 (impl)
macro_rules! Depcrate_astimpl_93 {
() => {
// Module: crate::ast
// Provides: {"impl_93"}
// Dependencies: {}
impl Parse for GlobalCtorDtor { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (GlobalCtorDtor , IndexStr < 'b >) > { try_begin_parse ! ("GlobalCtorDtor" , ctx , input) ; let tail = match input . next_or (error :: Error :: UnexpectedEnd) ? { (b'_' , t) | (b'.' , t) | (b'$' , t) => t , _ => return Err (error :: Error :: UnexpectedText) , } ; match tail . next_or (error :: Error :: UnexpectedEnd) ? { (b'I' , tail) => { let tail = consume (b"_" , tail) ? ; let (name , tail) = MangledName :: parse (ctx , subs , tail) ? ; Ok ((GlobalCtorDtor :: Ctor (Box :: new (name)) , tail)) } (b'D' , tail) => { let tail = consume (b"_" , tail) ? ; let (name , tail) = MangledName :: parse (ctx , subs , tail) ? ; Ok ((GlobalCtorDtor :: Dtor (Box :: new (name)) , tail)) } _ => Err (error :: Error :: UnexpectedText) , } } }
};
}
