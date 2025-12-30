// Generated macro for scheme_to_port (function)
macro_rules! Depcrate_connect_urischeme_to_port {
() => {
// Module: crate::connect::uri
// Provides: {"scheme_to_port"}
// Dependencies: {}
fn scheme_to_port (scheme : Option < & str >) -> Option < u16 > { match scheme { Some ("http") => Some (80) , Some ("https") => Some (443) , Some ("ws") => Some (80) , Some ("wss") => Some (443) , Some ("amqp") => Some (5672) , Some ("amqps") => Some (5671) , Some ("mqtt") => Some (1883) , Some ("mqtts") => Some (8883) , Some ("ftp") => Some (21) , Some ("ftps") => Some (990) , Some ("redis") => Some (6379) , Some ("mysql") => Some (3306) , Some ("postgres") => Some (5432) , _ => None , } }
};
}
