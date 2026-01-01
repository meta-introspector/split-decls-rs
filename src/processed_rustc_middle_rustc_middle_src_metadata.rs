/* FP:metadata.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_metadata_USE_0001
/* FP:metadata.rs-0002 */ use crate :: rustc_complete :: def :: Res ;
/* FP:metadata.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_metadata_USE_0002
/* FP:metadata.rs-0004 */ use rustc_macros :: { HashStable , TyDecodable , TyEncodable } ;
/* FP:metadata.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_metadata_USE_0003
/* FP:metadata.rs-0006 */ use crate :: rustc_complete :: Ident ;
/* FP:metadata.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_metadata_USE_0004
/* FP:metadata.rs-0008 */ use crate :: rustc_complete :: def_id :: DefId ;
/* FP:metadata.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_metadata_USE_0005
/* FP:metadata.rs-0010 */ use smallvec :: SmallVec ;
/* FP:metadata.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_metadata_USE_0006
/* FP:metadata.rs-0012 */ use crate :: ty ;
/* FP:metadata.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_metadata_ENUM_0007
/* FP:metadata.rs-0014 */ # [doc = " A simplified version of `ImportKind` from resolve."] # [doc = " `DefId`s here correspond to `use` and `extern crate` items themselves, not their targets."] # [derive (Clone , Copy , Debug , TyEncodable , TyDecodable , HashStable)] pub enum Reexport { Single (DefId) , Glob (DefId) , ExternCrate (DefId) , MacroUse , MacroExport , }
/* FP:metadata.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_metadata_IMPL_0008
/* FP:metadata.rs-0016 */ impl Reexport { pub fn id (self) -> Option < DefId > { match self { Reexport :: Single (id) | Reexport :: Glob (id) | Reexport :: ExternCrate (id) => Some (id) , Reexport :: MacroUse | Reexport :: MacroExport => None , } } }
/* FP:metadata.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_metadata_STRUCT_0009
/* FP:metadata.rs-0018 */ # [doc = " This structure is supposed to keep enough data to re-create `NameBinding`s for other crates"] # [doc = " during name resolution. Right now the bindings are not recreated entirely precisely so we may"] # [doc = " need to add more data in the future to correctly support macros 2.0, for example."] # [doc = " Module child can be either a proper item or a reexport (including private imports)."] # [doc = " In case of reexport all the fields describe the reexport item itself, not what it refers to."] # [derive (Debug , TyEncodable , TyDecodable , HashStable)] pub struct ModChild { # [doc = " Name of the item."] pub ident : Ident , # [doc = " Resolution result corresponding to the item."] # [doc = " Local variables cannot be exported, so this `Res` doesn't need the ID parameter."] pub res : Res < ! > , # [doc = " Visibility of the item."] pub vis : ty :: Visibility < DefId > , # [doc = " Reexport chain linking this module child to its original reexported item."] # [doc = " Empty if the module child is a proper item."] pub reexport_chain : SmallVec < [Reexport ; 2] > , }