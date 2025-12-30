// Generated macro for VCodeConstants (struct)
macro_rules! Depcrate_machinst_vcodeVCodeConstants {
() => {
// Module: crate::machinst::vcode
// Provides: {"VCodeConstants"}
// Dependencies: {}
# [doc = " This structure tracks the large constants used in VCode that will be emitted separately by the"] # [doc = " [MachBuffer]."] # [doc = ""] # [doc = " First, during the lowering phase, constants are inserted using"] # [doc = " [VCodeConstants.insert]; an intermediate handle, `VCodeConstant`, tracks what constants are"] # [doc = " used in this phase. Some deduplication is performed, when possible, as constant"] # [doc = " values are inserted."] # [doc = ""] # [doc = " Secondly, during the emission phase, the [MachBuffer] assigns [MachLabel]s for each of the"] # [doc = " constants so that instructions can refer to the value's memory location. The [MachBuffer]"] # [doc = " then writes the constant values to the buffer."] # [derive (Default)] pub struct VCodeConstants { constants : PrimaryMap < VCodeConstant , VCodeConstantData > , pool_uses : HashMap < Constant , VCodeConstant > , well_known_uses : HashMap < * const [u8] , VCodeConstant > , u64s : HashMap < [u8 ; 8] , VCodeConstant > , }
};
}
