// Generated macro for Element (enum)
macro_rules! Depcrate_dataElement {
() => {
// Module: crate::data
// Provides: {"Element"}
// Dependencies: {}
# [doc = " A graph element."] # [doc = ""] # [doc = " A sequence of Elements, for example an iterator, is laid out as follows:"] # [doc = " Nodes are implicitly given the index of their appearance in the sequence."] # [doc = " The edges’ source and target fields refer to these indices."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum Element < N , E > { # [doc = " A graph node."] Node { weight : N } , # [doc = " A graph edge."] Edge { source : usize , target : usize , weight : E , } , }
};
}
