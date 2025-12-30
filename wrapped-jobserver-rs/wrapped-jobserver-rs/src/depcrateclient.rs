// Generated macro for Client (struct)
macro_rules! DepcrateClient {
() => {
// Module: crate
// Provides: {"Client"}
// Dependencies: {}
# [doc = " A client of a jobserver"] # [doc = ""] # [doc = " This structure is the main type exposed by this library, and is where"] # [doc = " interaction to a jobserver is configured through. Clients are either created"] # [doc = " from scratch in which case the internal semphore is initialied on the spot,"] # [doc = " or a client is created from the environment to connect to a jobserver"] # [doc = " already created."] # [doc = ""] # [doc = " Some usage examples can be found in the crate documentation for using a"] # [doc = " client."] # [doc = ""] # [doc = " Note that a [`Client`] implements the [`Clone`] trait, and all instances of"] # [doc = " a [`Client`] refer to the same jobserver instance."] # [derive (Clone , Debug)] pub struct Client { inner : Arc < imp :: Client > , }
};
}
