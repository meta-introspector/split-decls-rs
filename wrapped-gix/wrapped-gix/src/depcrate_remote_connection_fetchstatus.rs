// Generated macro for Status (enum)
macro_rules! Depcrate_remote_connection_fetchStatus {
() => {
// Module: crate::remote::connection::fetch
// Provides: {"Status"}
// Dependencies: {}
# [doc = " The status of the repository after the fetch operation"] # [derive (Debug , Clone)] pub enum Status { # [doc = " Nothing changed as the remote didn't have anything new compared to our tracking branches, thus no pack was received"] # [doc = " and no new object was added."] # [doc = ""] # [doc = " As we could determine that nothing changed without remote interaction, there was no negotiation at all."] NoPackReceived { # [doc = " If `true`, we didn't receive a pack due to dry-run mode being enabled."] dry_run : bool , # [doc = " Information about the pack negotiation phase if negotiation happened at all."] # [doc = ""] # [doc = " It's possible that negotiation didn't have to happen as no reference of interest changed on the server."] negotiate : Option < outcome :: Negotiate > , # [doc = " However, depending on the refspecs, references might have been updated nonetheless to point to objects as"] # [doc = " reported by the remote."] update_refs : refs :: update :: Outcome , } , # [doc = " There was at least one tip with a new object which we received."] Change { # [doc = " Information about the pack negotiation phase."] negotiate : outcome :: Negotiate , # [doc = " Information collected while writing the pack and its index."] write_pack_bundle : gix_pack :: bundle :: write :: Outcome , # [doc = " Information collected while updating references."] update_refs : refs :: update :: Outcome , } , }
};
}
