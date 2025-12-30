// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl FromStr for Parameters { type Err = Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let (_ , params) = PARAMETERS_REGEX . captures (s) . ok_or_eyre ("Parameters regex didn't match") ? . extract :: < 9 > () ; Ok (Self { width : params [0] . parse () ? , poly : u128 :: from_str_radix (params [1] , 16) ? , init : u128 :: from_str_radix (params [2] , 16) ? , refin : parse_bool (params [3]) ? , refout : parse_bool (params [4]) ? , xorout : u128 :: from_str_radix (params [5] , 16) ? , check : u128 :: from_str_radix (params [6] , 16) ? , residue : u128 :: from_str_radix (params [7] , 16) ? , name : params [8] . to_owned () , }) } }
};
}
