// Generated macro for Transitions (struct)
macro_rules! Depcrate_dfaTransitions {
() => {
// Module: crate::dfa
// Provides: {"Transitions"}
// Dependencies: {}
# [doc = " The transition table."] # [doc = ""] # [doc = " It is laid out in row-major order, with states as rows and byte class"] # [doc = " transitions as columns."] # [doc = ""] # [doc = " The transition table is responsible for producing valid StatePtrs. A"] # [doc = " StatePtr points to the start of a particular row in this table. When"] # [doc = " indexing to find the next state this allows us to avoid a multiplication"] # [doc = " when computing an index into the table."] # [derive (Clone)] struct Transitions { # [doc = " The table."] table : Vec < StatePtr > , # [doc = " The stride."] num_byte_classes : usize , }
};
}
