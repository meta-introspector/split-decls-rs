// Generated macro for PubStuffParser (struct)
macro_rules! Depcrate_read_lookupPubStuffParser {
() => {
// Module: crate::read::lookup
// Provides: {"PubStuffParser"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct PubStuffParser < R , Entry > where R : Reader , Entry : PubStuffEntry < R > , { phantom : PhantomData < (R , Entry) > , }
};
}
