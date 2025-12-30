// Generated macro for cast (function)
macro_rules! Depcrate_castcast {
() => {
// Module: crate::cast
// Provides: {"cast"}
// Dependencies: {}
# [doc = " Cast from one machine scalar to another."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use num_traits as num;"] # [doc = " let twenty: f32 = num::cast(0x14).unwrap();"] # [doc = " assert_eq!(twenty, 20f32);"] # [doc = " ```"] # [doc = ""] # [inline] pub fn cast < T : NumCast , U : NumCast > (n : T) -> Option < U > { NumCast :: from (n) }
};
}
