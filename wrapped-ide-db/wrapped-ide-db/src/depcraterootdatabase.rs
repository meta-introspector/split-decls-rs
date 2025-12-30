// Generated macro for RootDatabase (struct)
macro_rules! DepcrateRootDatabase {
() => {
// Module: crate
// Provides: {"RootDatabase"}
// Dependencies: {}
# [salsa_macros :: db] pub struct RootDatabase { storage : ManuallyDrop < salsa :: Storage < Self > > , files : Arc < Files > , crates_map : Arc < CratesMap > , nonce : Nonce , }
};
}
