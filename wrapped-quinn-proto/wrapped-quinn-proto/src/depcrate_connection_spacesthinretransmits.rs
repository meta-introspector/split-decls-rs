// Generated macro for ThinRetransmits (struct)
macro_rules! Depcrate_connection_spacesThinRetransmits {
() => {
// Module: crate::connection::spaces
// Provides: {"ThinRetransmits"}
// Dependencies: {}
# [doc = " A variant of `Retransmits` which only allocates storage when required"] # [derive (Debug , Default , Clone)] pub (super) struct ThinRetransmits { retransmits : Option < Box < Retransmits > > , }
};
}
