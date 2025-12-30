// Generated macro for ConnectionIdentifiers (struct)
macro_rules! Depcrate_cidConnectionIdentifiers {
() => {
// Module: crate::cid
// Provides: {"ConnectionIdentifiers"}
// Dependencies: {}
# [derive (Default)] pub struct ConnectionIdentifiers { # [doc = " All the Destination Connection IDs provided by our peer."] dcids : BoundedNonEmptyConnectionIdVecDeque , # [doc = " All the Source Connection IDs we provide to our peer."] scids : BoundedNonEmptyConnectionIdVecDeque , # [doc = " Source Connection IDs that should be announced to the peer."] advertise_new_scid_seqs : VecDeque < u64 > , # [doc = " Retired Destination Connection IDs that should be announced to the peer."] retire_dcid_seqs : BoundedConnectionIdSeqSet , # [doc = " Retired Source Connection IDs that should be notified to the"] # [doc = " application."] retired_scids : VecDeque < ConnectionId < 'static > > , # [doc = " Largest \"Retire Prior To\" we received from the peer."] largest_peer_retire_prior_to : u64 , # [doc = " Largest sequence number we received from the peer."] largest_destination_seq : u64 , # [doc = " Next sequence number to use."] next_scid_seq : u64 , # [doc = " \"Retire Prior To\" value to advertise to the peer."] retire_prior_to : u64 , # [doc = " The maximum number of source Connection IDs our peer allows us."] source_conn_id_limit : usize , # [doc = " Does the host use zero-length source Connection ID."] zero_length_scid : bool , # [doc = " Does the host use zero-length destination Connection ID."] zero_length_dcid : bool , }
};
}
