// Generated macro for is_value_use_root (function)
macro_rules! Depcrate_machinst_loweris_value_use_root {
() => {
// Module: crate::machinst::lower
// Provides: {"is_value_use_root"}
// Dependencies: {}
# [doc = " Definition of a \"root\" instruction for the calculation of `ValueUseState`."] # [doc = ""] # [doc = " This function calculates whether `inst` is considered a \"root\" for value-use"] # [doc = " information. This concept is used to forcibly prevent looking-through the"] # [doc = " instruction during `get_value_as_source_or_const` as it additionally"] # [doc = " prevents propagating `Multiple`-used results of the `inst` here to the"] # [doc = " operands of the instruction."] # [doc = ""] # [doc = " Currently this is defined as multi-result instructions. That means that"] # [doc = " lowerings are never allowed to look through a multi-result instruction to"] # [doc = " generate patterns. Note that this isn't possible in ISLE today anyway so"] # [doc = " this isn't currently much of a loss."] # [doc = ""] # [doc = " The main purpose of this function is to prevent the operands of a"] # [doc = " multi-result instruction from being forcibly considered `Multiple`-used"] # [doc = " regardless of circumstances."] fn is_value_use_root (f : & Function , inst : Inst) -> bool { f . dfg . inst_results (inst) . len () > 1 }
};
}
