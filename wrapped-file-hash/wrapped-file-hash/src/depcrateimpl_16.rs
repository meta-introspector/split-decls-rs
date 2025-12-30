// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl Worker for HashWorker { type Input = HashInput ; type Output = HashOutput ; type Message = () ; fn create (_scope : & WorkerScope < Self >) -> Self { Self { } } fn connected (& mut self , _scope : & WorkerScope < Self > , _id : HandlerId) { } fn update (& mut self , _scope : & WorkerScope < Self > , _msg : Self :: Message) { } fn received (& mut self , scope : & WorkerScope < Self > , msg : Self :: Input , id : HandlerId) { let scope = scope . clone () ; spawn_local (async move { let mut hasher = Sha256 :: new () ; let mut s = ReadableStream :: from_raw (msg . file . stream () . unchecked_into ()) . into_stream () ; while let Some (chunk) = s . try_next () . await . unwrap () { hasher . update (chunk . unchecked_into :: < Uint8Array > () . to_vec ()) ; } let hash = hasher . finalize () ; let hash_hex = hex :: encode (hash) ; scope . respond (id , HashOutput { hash : hash_hex }) ; }) ; } }
};
}
