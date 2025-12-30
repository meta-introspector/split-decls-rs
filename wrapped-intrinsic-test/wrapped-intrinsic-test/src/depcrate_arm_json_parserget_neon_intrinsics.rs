// Generated macro for get_neon_intrinsics (function)
macro_rules! Depcrate_arm_json_parserget_neon_intrinsics {
() => {
// Module: crate::arm::json_parser
// Provides: {"get_neon_intrinsics"}
// Dependencies: {}
pub fn get_neon_intrinsics (filename : & Path , target : & str ,) -> Result < Vec < Intrinsic < ArmIntrinsicType > > , Box < dyn std :: error :: Error > > { let file = std :: fs :: File :: open (filename) ? ; let reader = std :: io :: BufReader :: new (file) ; let json : Vec < JsonIntrinsic > = serde_json :: from_reader (reader) . expect ("Couldn't parse JSON") ; let parsed = json . into_iter () . filter_map (| intr | { if intr . simd_isa == "Neon" { Some (json_to_intrinsic (intr , target) . expect ("Couldn't parse JSON")) } else { None } }) . collect () ; Ok (parsed) }
};
}
