// Generated macro for tests (module)
macro_rules! Depcrate_ir_memflagstests {
() => {
// Module: crate::ir::memflags
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn roundtrip_traps () { for trap in TrapCode :: non_user_traps () . iter () . copied () { let flags = MemFlags :: new () . with_trap_code (Some (trap)) ; assert_eq ! (flags . trap_code () , Some (trap)) ; } let flags = MemFlags :: new () . with_trap_code (None) ; assert_eq ! (flags . trap_code () , None) ; } # [test] fn cannot_set_big_and_little () { let mut big = MemFlags :: new () . with_endianness (Endianness :: Big) ; assert ! (big . set_by_name ("little") . is_err ()) ; let mut little = MemFlags :: new () . with_endianness (Endianness :: Little) ; assert ! (little . set_by_name ("big") . is_err ()) ; } # [test] fn only_one_region () { let mut big = MemFlags :: new () . with_alias_region (Some (AliasRegion :: Heap)) ; assert ! (big . set_by_name ("table") . is_err ()) ; assert ! (big . set_by_name ("vmctx") . is_err ()) ; let mut big = MemFlags :: new () . with_alias_region (Some (AliasRegion :: Table)) ; assert ! (big . set_by_name ("heap") . is_err ()) ; assert ! (big . set_by_name ("vmctx") . is_err ()) ; let mut big = MemFlags :: new () . with_alias_region (Some (AliasRegion :: Vmctx)) ; assert ! (big . set_by_name ("heap") . is_err ()) ; assert ! (big . set_by_name ("table") . is_err ()) ; } }
};
}
