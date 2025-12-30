// Generated macro for impl_14 (impl)
macro_rules! Depcrate_availabilityimpl_14 {
() => {
// Module: crate::availability
// Provides: {"impl_14"}
// Dependencies: {}
impl Versions { const NONE : Self = Self { macos : None , maccatalyst : None , ios : None , tvos : None , watchos : None , visionos : None , } ; const MIN : Self = { let min = Version { x : 0 , y : None , z : None , } ; Self { macos : Some (min) , maccatalyst : Some (min) , ios : Some (min) , tvos : Some (min) , watchos : Some (min) , visionos : Some (min) , } } ; const RUST_OS_MIN : Self = Self { macos : Some (Version { x : 10 , y : Some (12) , z : None , }) , maccatalyst : Some (Version { x : 13 , y : Some (1) , z : None , }) , ios : Some (Version { x : 10 , y : Some (0) , z : None , }) , tvos : Some (Version { x : 10 , y : Some (0) , z : None , }) , watchos : Some (Version { x : 5 , y : Some (0) , z : None , }) , visionos : Some (Version { x : 1 , y : Some (0) , z : None , }) , } ; fn emit_if (& self , bound : & Self , condition : impl Fn (Version , Version) -> bool) -> Self { let filter = | this , bound | { if let Some (this) = this { if let Some (bound) = bound { if (condition) (this , bound) { Some (this) } else { None } } else { Some (this) } } else { None } } ; Self { macos : filter (self . macos , bound . macos) , maccatalyst : filter (self . maccatalyst , bound . maccatalyst) , ios : filter (self . ios , bound . ios) , tvos : filter (self . tvos , bound . tvos) , watchos : filter (self . watchos , bound . watchos) , visionos : filter (self . visionos , bound . visionos) , } } }
};
}
