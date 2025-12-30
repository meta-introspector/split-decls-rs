// Generated macro for partition_module (macro)
macro_rules! Depcratepartition_module {
() => {
// Module: crate
// Provides: {"partition_module"}
// Dependencies: {}
macro_rules ! partition_module { ($ mod_name : ident , $ file_path : expr) => { pub mod $ mod_name { use llm_macros :: { llm_error_message , llm_context } ; include ! ("output2/wrapped-mockall_derive/src/decls/demutify_arg.rs") ; use quote ::*; use syn :: { FnArg , Generics , Ident , Pat , PatType , Signature , Token , Type , TypeParamBound , WherePredicate , GenericParam , parse2 } ; use syn :: punctuated :: Punctuated ; use proc_macro2 :: TokenStream ; # [derive (Hash , Eq , PartialEq , Clone)] pub struct Bom ; pub struct DeclContext ; pub struct DeclOrigin ; pub trait HirDatabase { } include ! (concat ! ("/home/mdupont/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/" , $ file_path)) ; } pub use $ mod_name ::*; } ; }
};
}
