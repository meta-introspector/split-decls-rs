// Generated macro for IncomingMessage (enum)
macro_rules! Depcrate_connectionIncomingMessage {
() => {
// Module: crate::connection
// Provides: {"IncomingMessage"}
// Dependencies: {}
# [doc = " Enum defining the messages we can receive"] # [derive (Debug , Deserialize)] pub enum IncomingMessage { FormatValue { value : f64 , } , FormatThroughput { value : f64 , throughput : Throughput , } , ScaleValues { typical_value : f64 , values : Vec < f64 > , } , ScaleThroughputs { typical_value : f64 , values : Vec < f64 > , throughput : Throughput , } , ScaleForMachines { values : Vec < f64 > , } , Continue , __Other , }
};
}
