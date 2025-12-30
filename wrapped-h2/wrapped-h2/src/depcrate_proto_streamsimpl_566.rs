// Generated macro for impl_566 (impl)
macro_rules! Depcrate_proto_streamsimpl_566 {
() => {
// Module: crate::proto::streams
// Provides: {"impl_566"}
// Dependencies: {}
impl < 'a , 'b > DebugStructExt < 'a , 'b > for std :: fmt :: DebugStruct < 'a , 'b > { fn h2_field_if (& mut self , name : & str , val : & bool) -> & mut std :: fmt :: DebugStruct < 'a , 'b > { if * val { self . field (name , val) } else { self } } fn h2_field_if_then < T : std :: fmt :: Debug > (& mut self , name : & str , cond : bool , val : & T ,) -> & mut std :: fmt :: DebugStruct < 'a , 'b > { if cond { self . field (name , val) } else { self } } fn h2_field_some < T : std :: fmt :: Debug > (& mut self , name : & str , val : & Option < T > ,) -> & mut std :: fmt :: DebugStruct < 'a , 'b > { if val . is_some () { self . field (name , val) } else { self } } }
};
}
