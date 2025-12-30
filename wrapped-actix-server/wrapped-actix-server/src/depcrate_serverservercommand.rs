// Generated macro for ServerCommand (enum)
macro_rules! Depcrate_serverServerCommand {
() => {
// Module: crate::server
// Provides: {"ServerCommand"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum ServerCommand { # [doc = " Worker failed to accept connection, indicating a probable panic."] # [doc = ""] # [doc = " Contains index of faulted worker."] WorkerFaulted (usize) , # [doc = " Pause accepting connections."] # [doc = ""] # [doc = " Contains return channel to notify caller of successful state change."] Pause (oneshot :: Sender < () >) , # [doc = " Resume accepting connections."] # [doc = ""] # [doc = " Contains return channel to notify caller of successful state change."] Resume (oneshot :: Sender < () >) , # [doc = " Stop accepting connections and begin shutdown procedure."] Stop { # [doc = " True if shut down should be graceful."] graceful : bool , # [doc = " Return channel to notify caller that shutdown is complete."] completion : Option < oneshot :: Sender < () > > , # [doc = " Force System exit when true, overriding `ServerBuilder::system_exit()` if it is false."] force_system_stop : bool , } , }
};
}
