// Generated macro for cast_target_to_abi_params (function)
macro_rules! Depcrate_abi_pass_modecast_target_to_abi_params {
() => {
// Module: crate::abi::pass_mode
// Provides: {"cast_target_to_abi_params"}
// Dependencies: {}
fn cast_target_to_abi_params (cast : & CastTarget) -> SmallVec < [(Size , AbiParam) ; 2] > { if let Some (offset_from_start) = cast . rest_offset { assert ! (cast . prefix [1 ..] . iter () . all (| p | p . is_none ())) ; assert_eq ! (cast . rest . unit . size , cast . rest . total) ; let first = cast . prefix [0] . unwrap () ; let second = cast . rest . unit ; return smallvec ! [(Size :: ZERO , reg_to_abi_param (first)) , (offset_from_start , reg_to_abi_param (second))] ; } let (rest_count , rem_bytes) = if cast . rest . unit . size . bytes () == 0 { (0 , 0) } else { (cast . rest . total . bytes () / cast . rest . unit . size . bytes () , cast . rest . total . bytes () % cast . rest . unit . size . bytes () ,) } ; let args = cast . prefix . iter () . flatten () . map (| & reg | reg_to_abi_param (reg)) . chain ((0 .. rest_count) . map (| _ | reg_to_abi_param (cast . rest . unit))) ; let mut res = SmallVec :: new () ; let mut offset = Size :: ZERO ; for arg in args { res . push ((offset , arg)) ; offset += Size :: from_bytes (arg . value_type . bytes ()) ; } if rem_bytes != 0 { assert_eq ! (cast . rest . unit . kind , RegKind :: Integer) ; res . push ((offset , reg_to_abi_param (Reg { kind : RegKind :: Integer , size : Size :: from_bytes (rem_bytes) }) ,)) ; } res }
};
}
