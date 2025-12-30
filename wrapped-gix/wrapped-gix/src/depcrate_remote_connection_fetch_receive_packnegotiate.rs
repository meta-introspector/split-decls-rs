// Generated macro for Negotiate (struct)
macro_rules! Depcrate_remote_connection_fetch_receive_packNegotiate {
() => {
// Module: crate::remote::connection::fetch::receive_pack
// Provides: {"Negotiate"}
// Dependencies: {}
struct Negotiate < 'a , 'b , 'c > { objects : & 'a crate :: OdbHandle , refs : & 'a gix_ref :: file :: Store , graph : & 'a mut gix_negotiate :: Graph < 'b , 'c > , alternates : Vec < PathBuf > , ref_map : & 'a gix_protocol :: fetch :: RefMap , shallow : & 'a gix_protocol :: fetch :: Shallow , tags : gix_protocol :: fetch :: Tags , negotiator : Box < dyn gix_negotiate :: Negotiator > , open_options : crate :: open :: Options , }
};
}
