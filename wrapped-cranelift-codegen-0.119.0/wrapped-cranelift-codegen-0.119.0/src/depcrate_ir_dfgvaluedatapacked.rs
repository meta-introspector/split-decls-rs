// Generated macro for ValueDataPacked (struct)
macro_rules! Depcrate_ir_dfgValueDataPacked {
() => {
// Module: crate::ir::dfg
// Provides: {"ValueDataPacked"}
// Dependencies: {}
# [doc = " Bit-packed version of ValueData, for efficiency."] # [doc = ""] # [doc = " Layout:"] # [doc = ""] # [doc = " ```plain"] # [doc = "        | tag:2 |  type:14        |    x:24       | y:24          |"] # [doc = ""] # [doc = " Inst       00     ty               inst output     inst index"] # [doc = " Param      01     ty               blockparam num  block index"] # [doc = " Alias      10     ty               0               value index"] # [doc = " Union      11     ty               first value     second value"] # [doc = " ```"] # [derive (Clone , Copy , Debug , PartialEq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] struct ValueDataPacked (u64) ;
};
}
