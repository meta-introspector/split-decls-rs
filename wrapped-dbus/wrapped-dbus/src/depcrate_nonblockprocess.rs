// Generated macro for Process (trait)
macro_rules! Depcrate_nonblockProcess {
() => {
// Module: crate::nonblock
// Provides: {"Process"}
// Dependencies: {}
# [doc = " Internal helper trait, implemented for connections that process incoming messages."] pub trait Process : Sender + AsRef < Channel > { # [doc = " Dispatches all pending messages, without blocking."] # [doc = ""] # [doc = " This is usually called from the reactor only, after read_write."] # [doc = " Despite this taking &self and not \"&mut self\", it is a logic error to call this"] # [doc = " recursively or from more than one thread at a time."] fn process_all (& self) { let c : & Channel = self . as_ref () ; while let Some (msg) = c . pop_message () { self . process_one (msg) ; } } # [doc = " Dispatches a message."] fn process_one (& self , msg : Message) ; }
};
}
