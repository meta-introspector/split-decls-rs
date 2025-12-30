// Generated macro for define_passes (macro)
macro_rules! Depcrate_timingdefine_passes {
() => {
// Module: crate::timing
// Provides: {"define_passes"}
// Dependencies: {}
macro_rules ! define_passes { ($ ($ pass : ident : $ desc : expr ,) +) => { # [doc = " A single profiled pass."] # [allow (non_camel_case_types)] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Pass { $ (# [doc =$ desc] $ pass ,) + # [doc = " No active pass."] None , } # [doc = " The amount of profiled passes."] pub const NUM_PASSES : usize = Pass :: None as usize ; const DESCRIPTIONS : [& str ; NUM_PASSES] = [$ ($ desc) ,+] ; $ (# [doc =$ desc] # [must_use] pub fn $ pass () -> Box < dyn Any > { start_pass (Pass ::$ pass) }) + } }
};
}
