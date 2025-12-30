// Generated macro for DebugStructExt (trait)
macro_rules! Depcrate_proto_streamsDebugStructExt {
() => {
// Module: crate::proto::streams
// Provides: {"DebugStructExt"}
// Dependencies: {}
trait DebugStructExt < 'a , 'b > { fn h2_field_if (& mut self , name : & str , val : & bool) -> & mut std :: fmt :: DebugStruct < 'a , 'b > ; fn h2_field_if_then < T : std :: fmt :: Debug > (& mut self , name : & str , cond : bool , val : & T ,) -> & mut std :: fmt :: DebugStruct < 'a , 'b > ; fn h2_field_some < T : std :: fmt :: Debug > (& mut self , name : & str , val : & Option < T > ,) -> & mut std :: fmt :: DebugStruct < 'a , 'b > ; }
};
}
