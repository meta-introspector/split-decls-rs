// Generated macro for to_range (function)
macro_rules! Depcrate_stringto_range {
() => {
// Module: crate::string
// Provides: {"to_range"}
// Dependencies: {}
fn to_range (rep : & Repetition) -> Result < SizeRange , Error > { Ok (match (rep . min , rep . max) { (0 , Some (1)) => size_range (0 ..= 1) , (0 , None) => size_range (0 ..= 32) , (1 , None) => size_range (1 ..= 32) , (u32 :: MAX , Some (u32 :: MAX)) => { return unsupported ("Cannot have repetition of exactly u32::MAX") ; } (min , Some (max)) if min == max => size_range (min as usize) , (min , None) => { let max = if min < u32 :: MAX as u32 / 2 { min as usize * 2 } else { u32 :: MAX as usize } ; size_range ((min as usize) .. max) } (_ , Some (u32 :: MAX)) => { return unsupported ("Cannot have repetition max of u32::MAX") } (min , Some (max)) => size_range ((min as usize) .. (max as usize + 1)) , }) }
};
}
