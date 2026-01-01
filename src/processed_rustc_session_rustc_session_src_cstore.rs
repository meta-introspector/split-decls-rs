/* FP:cstore.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_USE_0001
/* FP:cstore.rs-0002 */ use std :: any :: Any ;
/* FP:cstore.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_USE_0002
/* FP:cstore.rs-0004 */ use std :: path :: PathBuf ;
/* FP:cstore.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_USE_0003
/* FP:cstore.rs-0006 */ use crate :: rustc_abi :: ExternAbi ;
/* FP:cstore.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_USE_0004
/* FP:cstore.rs-0008 */ use crate :: rustc_data_structures :: sync :: { self , AppendOnlyIndexVec , FreezeLock } ;
/* FP:cstore.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_USE_0005
/* FP:cstore.rs-0010 */ use crate :: rustc_complete :: attrs :: { CfgEntry , NativeLibKind , PeImportNameType } ;
/* FP:cstore.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_USE_0006
/* FP:cstore.rs-0012 */ use crate :: rustc_complete :: def_id :: { CrateNum , DefId , LOCAL_CRATE , LocalDefId , StableCrateId , StableCrateIdMap , } ;
/* FP:cstore.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_USE_0007
/* FP:cstore.rs-0014 */ use crate :: rustc_complete :: definitions :: { DefKey , DefPath , DefPathHash , Definitions } ;
/* FP:cstore.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_USE_0008
/* FP:cstore.rs-0016 */ use rustc_macros :: { Decodable , Encodable , HashStable_Generic } ;
/* FP:cstore.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_USE_0009
/* FP:cstore.rs-0018 */ use crate :: rustc_complete :: { Span , Symbol } ;
/* FP:cstore.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_USE_0010
/* FP:cstore.rs-0020 */ use crate :: search_paths :: PathKind ;
/* FP:cstore.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_STRUCT_0011
/* FP:cstore.rs-0022 */ # [doc = " Where a crate came from on the local filesystem. One of these three options"] # [doc = " must be non-None."] # [derive (PartialEq , Clone , Debug , HashStable_Generic , Encodable , Decodable)] pub struct CrateSource { pub dylib : Option < (PathBuf , PathKind) > , pub rlib : Option < (PathBuf , PathKind) > , pub rmeta : Option < (PathBuf , PathKind) > , pub sdylib_interface : Option < (PathBuf , PathKind) > , }
/* FP:cstore.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_IMPL_0012
/* FP:cstore.rs-0024 */ impl CrateSource { # [inline] pub fn paths (& self) -> impl Iterator < Item = & PathBuf > { self . dylib . iter () . chain (self . rlib . iter ()) . chain (self . rmeta . iter ()) . map (| p | & p . 0) } }
/* FP:cstore.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_ENUM_0013
/* FP:cstore.rs-0026 */ # [derive (Encodable , Decodable , Copy , Clone , Ord , PartialOrd , Eq , PartialEq , Debug)] # [derive (HashStable_Generic)] pub enum CrateDepKind { # [doc = " A dependency that is only used for its macros."] MacrosOnly , # [doc = " A dependency that is always injected into the dependency list and so"] # [doc = " doesn't need to be linked to an rlib, e.g., the injected panic runtime."] Implicit , # [doc = " A dependency that is required by an rlib version of this crate."] # [doc = " Ordinary `extern crate`s result in `Explicit` dependencies."] Explicit , }
/* FP:cstore.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_IMPL_0014
/* FP:cstore.rs-0028 */ impl CrateDepKind { # [inline] pub fn macros_only (self) -> bool { match self { CrateDepKind :: MacrosOnly => true , CrateDepKind :: Implicit | CrateDepKind :: Explicit => false , } } }
/* FP:cstore.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_ENUM_0015
/* FP:cstore.rs-0030 */ # [derive (Copy , Debug , PartialEq , Clone , Encodable , Decodable , HashStable_Generic)] pub enum LinkagePreference { RequireDynamic , RequireStatic , }
/* FP:cstore.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_STRUCT_0016
/* FP:cstore.rs-0032 */ # [derive (Debug , Encodable , Decodable , HashStable_Generic)] pub struct NativeLib { pub kind : NativeLibKind , pub name : Symbol , # [doc = " If packed_bundled_libs enabled, actual filename of library is stored."] pub filename : Option < Symbol > , pub cfg : Option < CfgEntry > , pub foreign_module : Option < DefId > , pub verbatim : Option < bool > , pub dll_imports : Vec < DllImport > , }
/* FP:cstore.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_IMPL_0017
/* FP:cstore.rs-0034 */ impl NativeLib { pub fn has_modifiers (& self) -> bool { self . verbatim . is_some () || self . kind . has_modifiers () } pub fn wasm_import_module (& self) -> Option < Symbol > { if self . kind == NativeLibKind :: WasmImportModule { Some (self . name) } else { None } } }
/* FP:cstore.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_STRUCT_0018
/* FP:cstore.rs-0036 */ # [derive (Clone , Debug , Encodable , Decodable , HashStable_Generic)] pub struct DllImport { pub name : Symbol , pub import_name_type : Option < PeImportNameType > , # [doc = " Calling convention for the function."] # [doc = ""] # [doc = " On x86_64, this is always `DllCallingConvention::C`; on i686, it can be any"] # [doc = " of the values, and we use `DllCallingConvention::C` to represent `\"cdecl\"`."] pub calling_convention : DllCallingConvention , # [doc = " Span of import's \"extern\" declaration; used for diagnostics."] pub span : Span , # [doc = " Is this for a function (rather than a static variable)."] pub is_fn : bool , }
/* FP:cstore.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_IMPL_0019
/* FP:cstore.rs-0038 */ impl DllImport { pub fn ordinal (& self) -> Option < u16 > { if let Some (PeImportNameType :: Ordinal (ordinal)) = self . import_name_type { Some (ordinal) } else { None } } pub fn is_missing_decorations (& self) -> bool { self . import_name_type == Some (PeImportNameType :: Undecorated) || self . import_name_type == Some (PeImportNameType :: NoPrefix) } }
/* FP:cstore.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_ENUM_0020
/* FP:cstore.rs-0040 */ # [doc = " Calling convention for a function defined in an external library."] # [doc = ""] # [doc = " The usize value, where present, indicates the size of the function's argument list"] # [doc = " in bytes."] # [derive (Clone , PartialEq , Debug , Encodable , Decodable , HashStable_Generic)] pub enum DllCallingConvention { C , Stdcall (usize) , Fastcall (usize) , Vectorcall (usize) , }
/* FP:cstore.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_STRUCT_0021
/* FP:cstore.rs-0042 */ # [derive (Clone , Encodable , Decodable , HashStable_Generic , Debug)] pub struct ForeignModule { pub foreign_items : Vec < DefId > , pub def_id : DefId , pub abi : ExternAbi , }
/* FP:cstore.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_STRUCT_0022
/* FP:cstore.rs-0044 */ # [derive (Copy , Clone , Debug , HashStable_Generic)] pub struct ExternCrate { pub src : ExternCrateSource , # [doc = " span of the extern crate that caused this to be loaded"] pub span : Span , # [doc = " Number of links to reach the extern;"] # [doc = " used to select the extern with the shortest path"] pub path_len : usize , # [doc = " Crate that depends on this crate"] pub dependency_of : CrateNum , }
/* FP:cstore.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_IMPL_0023
/* FP:cstore.rs-0046 */ impl ExternCrate { # [doc = " If true, then this crate is the crate named by the extern"] # [doc = " crate referenced above. If false, then this crate is a dep"] # [doc = " of the crate."] # [inline] pub fn is_direct (& self) -> bool { self . dependency_of == LOCAL_CRATE } # [inline] pub fn rank (& self) -> impl PartialOrd { (self . is_direct () , ! self . path_len) } }
/* FP:cstore.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_ENUM_0024
/* FP:cstore.rs-0048 */ # [derive (Copy , Clone , Debug , HashStable_Generic)] pub enum ExternCrateSource { # [doc = " Crate is loaded by `extern crate`."] Extern (# [doc = " def_id of the item in the current crate that caused"] # [doc = " this crate to be loaded; note that there could be multiple"] # [doc = " such ids"] DefId ,) , # [doc = " Crate is implicitly loaded by a path resolving through extern prelude."] Path , }
/* FP:cstore.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_TRAIT_0025
/* FP:cstore.rs-0050 */ # [doc = " A store of Rust crates, through which their metadata can be accessed."] # [doc = ""] # [doc = " Note that this trait should probably not be expanding today. All new"] # [doc = " functionality should be driven through queries instead!"] # [doc = ""] # [doc = " If you find a method on this trait named `{name}_untracked` it signifies"] # [doc = " that it's *not* tracked for dependency information throughout compilation"] # [doc = " (it'd break incremental compilation) and should only be called pre-HIR (e.g."] # [doc = " during resolve)"] pub trait CrateStore : std :: fmt :: Debug { fn as_any (& self) -> & dyn Any ; fn untracked_as_any (& mut self) -> & mut dyn Any ; fn def_key (& self , def : DefId) -> DefKey ; fn def_path (& self , def : DefId) -> DefPath ; fn def_path_hash (& self , def : DefId) -> DefPathHash ; fn crate_name (& self , cnum : CrateNum) -> Symbol ; fn stable_crate_id (& self , cnum : CrateNum) -> StableCrateId ; }
/* FP:cstore.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_TYPE_0026
/* FP:cstore.rs-0052 */ pub type CrateStoreDyn = dyn CrateStore + sync :: DynSync + sync :: DynSend ;
/* FP:cstore.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_cstore_STRUCT_0027
/* FP:cstore.rs-0054 */ pub struct Untracked { pub cstore : FreezeLock < Box < CrateStoreDyn > > , # [doc = " Reference span for definitions."] pub source_span : AppendOnlyIndexVec < LocalDefId , Span > , pub definitions : FreezeLock < Definitions > , # [doc = " The interned [StableCrateId]s."] pub stable_crate_ids : FreezeLock < StableCrateIdMap > , }