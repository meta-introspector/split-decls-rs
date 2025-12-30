// Generated macro for aes_round (function)
macro_rules! Depcrate_shims_x86_aesniaes_round {
() => {
// Module: crate::shims::x86::aesni
// Provides: {"aes_round"}
// Dependencies: {}
fn aes_round < 'tcx > (ecx : & mut crate :: MiriInterpCx < 'tcx > , state : & OpTy < 'tcx > , key : & OpTy < 'tcx > , dest : & MPlaceTy < 'tcx > , f : impl Fn (u128 , u128) -> u128 ,) -> InterpResult < 'tcx , () > { assert_eq ! (dest . layout . size , state . layout . size) ; assert_eq ! (dest . layout . size , key . layout . size) ; assert_eq ! (dest . layout . size . bytes () % 16 , 0) ; let len = dest . layout . size . bytes () / 16 ; let u128_array_layout = ecx . layout_of (Ty :: new_array (ecx . tcx . tcx , ecx . tcx . types . u128 , len)) ? ; let state = state . transmute (u128_array_layout , ecx) ? ; let key = key . transmute (u128_array_layout , ecx) ? ; let dest = dest . transmute (u128_array_layout , ecx) ? ; for i in 0 .. len { let state = ecx . read_scalar (& ecx . project_index (& state , i) ?) ? . to_u128 () ? ; let key = ecx . read_scalar (& ecx . project_index (& key , i) ?) ? . to_u128 () ? ; let dest = ecx . project_index (& dest , i) ? ; let res = f (state , key) ; ecx . write_scalar (Scalar :: from_u128 (res) , & dest) ? ; } interp_ok (()) }
};
}
