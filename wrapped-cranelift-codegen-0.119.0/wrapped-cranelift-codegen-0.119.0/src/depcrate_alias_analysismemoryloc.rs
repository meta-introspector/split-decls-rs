// Generated macro for MemoryLoc (struct)
macro_rules! Depcrate_alias_analysisMemoryLoc {
() => {
// Module: crate::alias_analysis
// Provides: {"MemoryLoc"}
// Dependencies: {}
# [doc = " A key identifying a unique memory location."] # [doc = ""] # [doc = " For the result of a load to be equivalent to the result of another"] # [doc = " load, or the store data from a store, we need for (i) the"] # [doc = " \"version\" of memory (here ensured by having the same last store"] # [doc = " instruction to touch the disjoint category of abstract state we're"] # [doc = " accessing); (ii) the address must be the same (here ensured by"] # [doc = " having the same SSA value, which doesn't change after computed);"] # [doc = " (iii) the offset must be the same; and (iv) the accessed type and"] # [doc = " extension mode (e.g., 8-to-32, signed) must be the same."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] struct MemoryLoc { last_store : PackedOption < Inst > , address : Value , offset : Offset32 , ty : Type , # [doc = " We keep the *opcode* of the instruction that produced the"] # [doc = " value we record at this key if the opcode is anything other"] # [doc = " than an ordinary load or store. This is needed when we"] # [doc = " consider loads that extend the value: e.g., an 8-to-32"] # [doc = " sign-extending load will produce a 32-bit value from an 8-bit"] # [doc = " value in memory, so we can only reuse that (as part of RLE)"] # [doc = " for another load with the same extending opcode."] # [doc = ""] # [doc = " We could improve the transform to insert explicit extend ops"] # [doc = " in place of extending loads when we know the memory value, but"] # [doc = " we haven't yet done this."] extending_opcode : Option < Opcode > , }
};
}
