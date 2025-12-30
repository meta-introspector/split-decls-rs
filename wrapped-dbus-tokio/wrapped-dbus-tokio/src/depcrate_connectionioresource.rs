// Generated macro for IOResource (struct)
macro_rules! Depcrate_connectionIOResource {
() => {
// Module: crate::connection
// Provides: {"IOResource"}
// Dependencies: {}
# [doc = " The I/O Resource should be spawned onto a Tokio compatible reactor."] # [doc = ""] # [doc = " If you need to ever cancel this resource (i e disconnect from D-Bus),"] # [doc = " you need to make this future abortable. If it finishes, you probably lost"] # [doc = " contact with the D-Bus server."] pub struct IOResource < C > { connection : Arc < C > , registration : IOResourceRegistration , wake : Arc < Mutex < WakeStatus > > , write_pending : bool , }
};
}
