// Generated macro for InteractSession (struct)
macro_rules! Depcrate_interact_sessionInteractSession {
() => {
// Module: crate::interact::session
// Provides: {"InteractSession"}
// Dependencies: {}
# [doc = " InteractConfig represents options of an interactive session."] pub struct InteractSession < Session , Input , Output , State > { session : Session , input : Input , output : Output , escape_character : u8 , # [cfg (unix)] status : Option < WaitStatus > , opts : InteractOptions < Session , Input , Output , State > , }
};
}
