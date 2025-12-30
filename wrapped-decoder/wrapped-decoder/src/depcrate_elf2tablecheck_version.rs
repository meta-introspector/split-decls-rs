// Generated macro for check_version (function)
macro_rules! Depcrate_elf2tablecheck_version {
() => {
// Module: crate::elf2table
// Provides: {"check_version"}
// Dependencies: {}
# [doc = " Checks if the version encoded in the symbol table is compatible with this version of the `decoder` crate"] fn check_version (version : & str) -> Result < () , String > { if ! DEFMT_VERSIONS . contains (& version) { let msg = format ! ("defmt wire format version mismatch: firmware is using {}, this tool supports {}\nsuggestion: install a newer version of this tool that supports defmt wire format version {}" , version , DEFMT_VERSIONS . join (", ") , version) ; return Err (msg) ; } Ok (()) }
};
}
