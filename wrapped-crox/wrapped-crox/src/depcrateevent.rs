// Generated macro for Event (struct)
macro_rules! DepcrateEvent {
() => {
// Module: crate
// Provides: {"Event"}
// Dependencies: {}
# [derive (Serialize)] struct Event { name : String , # [serde (rename = "cat")] category : String , # [serde (rename = "ph")] event_type : EventType , # [serde (rename = "ts" , serialize_with = "as_micros")] timestamp : Duration , # [serde (rename = "dur" , serialize_with = "as_micros")] duration : Duration , # [serde (rename = "pid")] process_id : u32 , # [serde (rename = "tid")] thread_id : u32 , args : Option < FxHashMap < String , String > > , }
};
}
