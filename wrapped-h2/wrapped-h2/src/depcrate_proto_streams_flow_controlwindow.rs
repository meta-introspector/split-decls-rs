// Generated macro for Window (struct)
macro_rules! Depcrate_proto_streams_flow_controlWindow {
() => {
// Module: crate::proto::streams::flow_control
// Provides: {"Window"}
// Dependencies: {}
# [doc = " The current capacity of a flow-controlled Window."] # [doc = ""] # [doc = " This number can go negative when either side has used a certain amount"] # [doc = " of capacity when the other side advertises a reduction in size."] # [doc = ""] # [doc = " This type tries to centralize the knowledge of addition and subtraction"] # [doc = " to this capacity, instead of having integer casts throughout the source."] # [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd)] pub struct Window (i32) ;
};
}
