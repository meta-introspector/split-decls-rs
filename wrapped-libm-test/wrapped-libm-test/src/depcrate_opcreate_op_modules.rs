// Generated macro for create_op_modules (macro)
macro_rules! Depcrate_opcreate_op_modules {
() => {
// Module: crate::op
// Provides: {"create_op_modules"}
// Dependencies: {}
macro_rules ! create_op_modules { (fn_name : $ fn_name : ident , FTy : $ FTy : ty , CFn : $ CFn : ty , CArgs : $ CArgs : ty , CRet : $ CRet : ty , RustFn : $ RustFn : ty , RustArgs : $ RustArgs : ty , RustRet : $ RustRet : ty , public : $ public : expr , attrs : [$ ($ attr : meta) ,*] ,) => { paste :: paste ! { $ (# [$ attr]) * pub mod $ fn_name { use super ::*; pub struct Routine ; impl MathOp for Routine { type FTy = $ FTy ; type CFn = for <'a > $ CFn ; type CArgs <'a > = $ CArgs where Self : 'a ; type CRet = $ CRet ; type RustFn = $ RustFn ; type RustArgs = $ RustArgs ; type RustRet = $ RustRet ; const IDENTIFIER : Identifier = Identifier :: [< $ fn_name : camel >] ; const ROUTINE : Self :: RustFn = libm ::$ fn_name ; const PUBLIC : bool = $ public ; } } } } ; }
};
}
