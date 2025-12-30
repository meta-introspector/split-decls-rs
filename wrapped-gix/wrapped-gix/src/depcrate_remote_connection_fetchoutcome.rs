// Generated macro for outcome (module)
macro_rules! Depcrate_remote_connection_fetchoutcome {
() => {
// Module: crate::remote::connection::fetch
// Provides: {"outcome"}
// Dependencies: {}
# [doc = " Additional types related to the outcome of a fetch operation."] pub mod outcome { # [doc = " Information about the negotiation phase of a fetch."] # [doc = ""] # [doc = " Note that negotiation can happen even if no pack is ultimately produced."] # [derive (Default , Debug , Clone)] pub struct Negotiate { # [doc = " The negotiation graph indicating what kind of information 'the algorithm' collected in the end."] pub graph : gix_negotiate :: IdMap , # [doc = " Additional information for each round of negotiation."] pub rounds : Vec < gix_protocol :: fetch :: negotiate :: Round > , } }
};
}
