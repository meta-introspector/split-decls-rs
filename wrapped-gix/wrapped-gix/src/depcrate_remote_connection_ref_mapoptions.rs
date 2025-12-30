// Generated macro for Options (struct)
macro_rules! Depcrate_remote_connection_ref_mapOptions {
() => {
// Module: crate::remote::connection::ref_map
// Provides: {"Options"}
// Dependencies: {}
# [doc = " For use in [`Connection::ref_map()`]."] # [derive (Debug , Clone)] pub struct Options { # [doc = " Use a two-component prefix derived from the ref-spec's source, like `refs/heads/`  to let the server pre-filter refs"] # [doc = " with great potential for savings in traffic and local CPU time. Defaults to `true`."] pub prefix_from_spec_as_filter_on_remote : bool , # [doc = " Parameters in the form of `(name, optional value)` to add to the handshake."] # [doc = ""] # [doc = " This is useful in case of custom servers."] pub handshake_parameters : Vec < (String , Option < String >) > , # [doc = " A list of refspecs to use as implicit refspecs which won't be saved or otherwise be part of the remote in question."] # [doc = ""] # [doc = " This is useful for handling `remote.<name>.tagOpt` for example."] pub extra_refspecs : Vec < gix_refspec :: RefSpec > , }
};
}
