// Generated macro for impl_472 (impl)
macro_rules! Depcrate_connection_spacesimpl_472 {
() => {
// Module: crate::connection::spaces
// Provides: {"impl_472"}
// Dependencies: {}
impl ThinRetransmits { # [doc = " Returns `true` if no retransmits are necessary"] pub (super) fn is_empty (& self , streams : & StreamsState) -> bool { match & self . retransmits { Some (retransmits) => retransmits . is_empty (streams) , None => true , } } # [doc = " Returns a reference to the retransmits stored in this box"] pub (super) fn get (& self) -> Option < & Retransmits > { self . retransmits . as_deref () } # [doc = " Returns a mutable reference to the stored retransmits"] # [doc = ""] # [doc = " This function will allocate a backing storage if required."] pub (super) fn get_or_create (& mut self) -> & mut Retransmits { if self . retransmits . is_none () { self . retransmits = Some (Box :: default ()) ; } self . retransmits . as_deref_mut () . unwrap () } }
};
}
