// Generated macro for declare_id_with_package_metadata (macro)
macro_rules! Depcratedeclare_id_with_package_metadata {
() => {
// Module: crate
// Provides: {"declare_id_with_package_metadata"}
// Dependencies: {}
# [doc = " Convenience macro for declaring a program id from Cargo.toml package metadata."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `key` - A string slice of a dot-separated path to the TOML key of interest"] # [doc = ""] # [doc = " # Example"] # [doc = " Given the following `Cargo.toml`:"] # [doc = " ```ignore"] # [doc = " [package]"] # [doc = " name = \"my-solana-program\""] # [doc = " version = \"0.1.0\""] # [doc = ""] # [doc = " [package.metadata.solana]"] # [doc = " program-id = \"MyProgram1111111111111111111111111111111111\""] # [doc = " ```"] # [doc = ""] # [doc = " A program can use the program id declared in its `Cargo.toml` as the program"] # [doc = " id in code:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " declare_id_with_package_metadata!(\"solana.program-id\");"] # [doc = " ```"] # [doc = ""] # [doc = " This program id behaves exactly as if the developer had written:"] # [doc = ""] # [doc = " ```"] # [doc = " solana_pubkey::declare_id!(\"MyProgram1111111111111111111111111111111111\");"] # [doc = " ```"] # [doc = ""] # [doc = " Meaning that it's possible to refer to the program id using `crate::id()`,"] # [doc = " without needing to specify the program id in multiple places."] # [macro_export] macro_rules ! declare_id_with_package_metadata { ($ key : literal) => { $ crate :: declare_id ! ($ crate :: package_metadata ! ($ key)) ; } ; }
};
}
