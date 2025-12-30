// Generated macro for RoundImm (enum)
macro_rules! Depcrate_isa_x64_inst_argsRoundImm {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"RoundImm"}
// Dependencies: {}
# [doc = " Encode the rounding modes used as part of the Rounding Control field."] # [doc = " Note, these rounding immediates only consider the rounding control field"] # [doc = " (i.e. the rounding mode) which only take up the first two bits when encoded."] # [doc = " However the rounding immediate which this field helps make up, also includes"] # [doc = " bits 3 and 4 which define the rounding select and precision mask respectively."] # [doc = " These two bits are not defined here and are implicitly set to zero when encoded."] # [derive (Clone , Copy)] pub enum RoundImm { # [doc = " Round to nearest mode."] RoundNearest = 0x00 , # [doc = " Round down mode."] RoundDown = 0x01 , # [doc = " Round up mode."] RoundUp = 0x02 , # [doc = " Round to zero mode."] RoundZero = 0x03 , }
};
}
