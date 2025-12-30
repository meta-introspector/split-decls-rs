// Generated macro for UnaryRmROpcode (enum)
macro_rules! Depcrate_isa_x64_inst_argsUnaryRmROpcode {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"UnaryRmROpcode"}
// Dependencies: {}
# [derive (Clone , PartialEq)] # [doc = " Unary operations requiring register or memory and register operands."] pub enum UnaryRmROpcode { # [doc = " Bit-scan reverse."] Bsr , # [doc = " Bit-scan forward."] Bsf , # [doc = " Counts leading zeroes (Leading Zero CouNT)."] Lzcnt , # [doc = " Counts trailing zeroes (Trailing Zero CouNT)."] Tzcnt , # [doc = " Counts the number of ones (POPulation CouNT)."] Popcnt , }
};
}
