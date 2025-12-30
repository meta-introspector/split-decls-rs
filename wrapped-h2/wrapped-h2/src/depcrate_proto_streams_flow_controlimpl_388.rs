// Generated macro for impl_388 (impl)
macro_rules! Depcrate_proto_streams_flow_controlimpl_388 {
() => {
// Module: crate::proto::streams::flow_control
// Provides: {"impl_388"}
// Dependencies: {}
impl Window { pub fn as_size (& self) -> WindowSize { if self . 0 < 0 { 0 } else { self . 0 as WindowSize } } pub fn checked_size (& self) -> WindowSize { assert ! (self . 0 >= 0 , "negative Window") ; self . 0 as WindowSize } pub fn decrease_by (& mut self , other : WindowSize) -> Result < () , Reason > { if let Some (v) = self . 0 . checked_sub (other as i32) { self . 0 = v ; Ok (()) } else { Err (Reason :: FLOW_CONTROL_ERROR) } } pub fn increase_by (& mut self , other : WindowSize) -> Result < () , Reason > { let other = self . add (other) ? ; self . 0 = other . 0 ; Ok (()) } pub fn add (& self , other : WindowSize) -> Result < Self , Reason > { if let Some (v) = self . 0 . checked_add (other as i32) { Ok (Self (v)) } else { Err (Reason :: FLOW_CONTROL_ERROR) } } }
};
}
