// Generated macro for lib_call_arg_param (function)
macro_rules! Depcrate_abilib_call_arg_param {
() => {
// Module: crate::abi
// Provides: {"lib_call_arg_param"}
// Dependencies: {}
pub (crate) fn lib_call_arg_param (tcx : TyCtxt < '_ > , ty : Type , is_signed : bool) -> AbiParam { let param = AbiParam :: new (ty) ; if ty . is_int () && u64 :: from (ty . bits ()) < tcx . data_layout . pointer_size () . bits () { match (& * tcx . sess . target . arch , & * tcx . sess . target . vendor) { ("x86_64" , _) | ("aarch64" , "apple") => match (ty , is_signed) { (types :: I8 | types :: I16 , true) => param . sext () , (types :: I8 | types :: I16 , false) => param . uext () , _ => param , } , ("aarch64" , _) => param , ("riscv64" , _) => match (ty , is_signed) { (types :: I32 , _) | (_ , true) => param . sext () , _ => param . uext () , } , ("s390x" , _) => { if is_signed { param . sext () } else { param . uext () } } _ => unimplemented ! ("{:?}" , tcx . sess . target . arch) , } } else { param } }
};
}
