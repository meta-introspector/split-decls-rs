// Generated macro for AdapterKind (enum)
macro_rules! Depcrate_wit_standardAdapterKind {
() => {
// Module: crate::wit::standard
// Provides: {"AdapterKind"}
// Dependencies: {}
# [derive (Debug , Clone)] pub enum AdapterKind { Local { instructions : Vec < InstructionData > , } , Import { name : String , kind : AdapterJsImportKind , } , }
};
}
