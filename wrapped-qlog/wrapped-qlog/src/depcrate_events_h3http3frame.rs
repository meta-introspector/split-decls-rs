// Generated macro for Http3Frame (enum)
macro_rules! Depcrate_events_h3Http3Frame {
() => {
// Module: crate::events::h3
// Provides: {"Http3Frame"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] # [serde (tag = "frame_type")] # [serde (rename_all = "snake_case")] pub enum Http3Frame { Data { raw : Option < RawInfo > , } , Headers { headers : Vec < HttpHeader > , } , CancelPush { push_id : u64 , } , Settings { settings : Vec < Setting > , } , PushPromise { push_id : u64 , headers : Vec < HttpHeader > , } , Goaway { id : u64 , } , MaxPushId { push_id : u64 , } , PriorityUpdate { target_stream_type : H3PriorityTargetStreamType , prioritized_element_id : u64 , priority_field_value : String , } , Reserved { length : Option < u64 > , } , Unknown { frame_type_value : u64 , raw : Option < RawInfo > , } , }
};
}
