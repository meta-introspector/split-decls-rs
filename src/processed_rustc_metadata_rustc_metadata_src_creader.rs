/* FP:creader.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0001
/* FP:creader.rs-0002 */ use std :: error :: Error ;
/* FP:creader.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0002
/* FP:creader.rs-0004 */ use std :: path :: Path ;
/* FP:creader.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0003
/* FP:creader.rs-0006 */ use std :: str :: FromStr ;
/* FP:creader.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0004
/* FP:creader.rs-0008 */ use std :: time :: Duration ;
/* FP:creader.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0005
/* FP:creader.rs-0010 */ use std :: { cmp , env , iter } ;
/* FP:creader.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0006
/* FP:creader.rs-0012 */ use crate :: rustc_complete :: expand :: allocator :: { AllocatorKind , alloc_error_handler_name , global_fn_name } ;
/* FP:creader.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0007
/* FP:creader.rs-0014 */ use crate :: rustc_complete :: { self as ast , * } ;
/* FP:creader.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0008
/* FP:creader.rs-0016 */ use crate :: rustc_data_structures :: fx :: FxHashSet ;
/* FP:creader.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0009
/* FP:creader.rs-0018 */ use crate :: rustc_data_structures :: owned_slice :: OwnedSlice ;
/* FP:creader.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0010
/* FP:creader.rs-0020 */ use crate :: rustc_data_structures :: svh :: Svh ;
/* FP:creader.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0011
/* FP:creader.rs-0022 */ use crate :: rustc_data_structures :: sync :: { self , FreezeReadGuard , FreezeWriteGuard } ;
/* FP:creader.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0012
/* FP:creader.rs-0024 */ use crate :: rustc_data_structures :: unord :: UnordMap ;
/* FP:creader.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0013
/* FP:creader.rs-0026 */ use crate :: rustc_expand :: base :: SyntaxExtension ;
/* FP:creader.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0014
/* FP:creader.rs-0028 */ use rustc_fs_util :: try_canonicalize ;
/* FP:creader.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0015
/* FP:creader.rs-0030 */ use rustc_hir as hir ;
/* FP:creader.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0016
/* FP:creader.rs-0032 */ use crate :: rustc_complete :: def_id :: { CrateNum , LOCAL_CRATE , LocalDefId , StableCrateId } ;
/* FP:creader.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0017
/* FP:creader.rs-0034 */ use crate :: rustc_complete :: definitions :: Definitions ;
/* FP:creader.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0018
/* FP:creader.rs-0036 */ use crate :: rustc_index :: IndexVec ;
/* FP:creader.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0019
/* FP:creader.rs-0038 */ use crate :: rustc_complete :: bug ;
/* FP:creader.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0020
/* FP:creader.rs-0040 */ use crate :: rustc_complete :: ty :: data_structures :: IndexSet ;
/* FP:creader.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0021
/* FP:creader.rs-0042 */ use crate :: rustc_complete :: ty :: { TyCtxt , TyCtxtFeed } ;
/* FP:creader.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0022
/* FP:creader.rs-0044 */ use crate :: rustc_proc_macro :: bridge :: client :: ProcMacro ;
/* FP:creader.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0023
/* FP:creader.rs-0046 */ use crate :: rustc_complete :: Session ;
/* FP:creader.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0024
/* FP:creader.rs-0048 */ use crate :: rustc_complete :: config :: { CrateType , ExtendedTargetModifierInfo , ExternLocation , Externs , OptionsTargetModifiers , TargetModifier , } ;
/* FP:creader.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0025
/* FP:creader.rs-0050 */ use crate :: rustc_complete :: cstore :: { CrateDepKind , CrateSource , ExternCrate , ExternCrateSource } ;
/* FP:creader.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0026
/* FP:creader.rs-0052 */ use crate :: rustc_complete :: lint :: { self , BuiltinLintDiag } ;
/* FP:creader.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0027
/* FP:creader.rs-0054 */ use crate :: rustc_complete :: output :: validate_crate_name ;
/* FP:creader.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0028
/* FP:creader.rs-0056 */ use crate :: rustc_complete :: search_paths :: PathKind ;
/* FP:creader.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0029
/* FP:creader.rs-0058 */ use crate :: rustc_complete :: def_id :: DefId ;
/* FP:creader.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0030
/* FP:creader.rs-0060 */ use crate :: rustc_complete :: edition :: Edition ;
/* FP:creader.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0031
/* FP:creader.rs-0062 */ use crate :: rustc_complete :: { DUMMY_SP , Ident , Span , Symbol , sym } ;
/* FP:creader.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0032
/* FP:creader.rs-0064 */ use crate :: rustc_target :: spec :: { PanicStrategy , Target } ;
/* FP:creader.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0033
/* FP:creader.rs-0066 */ use tracing :: { debug , info , trace } ;
/* FP:creader.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0034
/* FP:creader.rs-0068 */ use crate :: errors ;
/* FP:creader.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0035
/* FP:creader.rs-0070 */ use crate :: locator :: { CrateError , CrateLocator , CratePaths , CrateRejections } ;
/* FP:creader.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_USE_0036
/* FP:creader.rs-0072 */ use crate :: rmeta :: { CrateDep , CrateMetadata , CrateNumMap , CrateRoot , MetadataBlob , TargetModifiers , } ;
/* FP:creader.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_TRAIT_0037
/* FP:creader.rs-0074 */ # [doc = " The backend's way to give the crate store access to the metadata in a library."] # [doc = " Note that it returns the raw metadata bytes stored in the library file, whether"] # [doc = " it is compressed, uncompressed, some weird mix, etc."] # [doc = " rmeta files are backend independent and not handled here."] pub trait MetadataLoader { fn get_rlib_metadata (& self , target : & Target , filename : & Path) -> Result < OwnedSlice , String > ; fn get_dylib_metadata (& self , target : & Target , filename : & Path) -> Result < OwnedSlice , String > ; }
/* FP:creader.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_TYPE_0038
/* FP:creader.rs-0076 */ pub type MetadataLoaderDyn = dyn MetadataLoader + Send + Sync + sync :: DynSend + sync :: DynSync ;
/* FP:creader.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_STRUCT_0039
/* FP:creader.rs-0078 */ pub struct CStore { metadata_loader : Box < MetadataLoaderDyn > , metas : IndexVec < CrateNum , Option < Box < CrateMetadata > > > , injected_panic_runtime : Option < CrateNum > , # [doc = " This crate needs an allocator and either provides it itself, or finds it in a dependency."] # [doc = " If the above is true, then this field denotes the kind of the found allocator."] allocator_kind : Option < AllocatorKind > , # [doc = " This crate needs an allocation error handler and either provides it itself, or finds it in a dependency."] # [doc = " If the above is true, then this field denotes the kind of the found allocator."] alloc_error_handler_kind : Option < AllocatorKind > , # [doc = " This crate has a `#[global_allocator]` item."] has_global_allocator : bool , # [doc = " This crate has a `#[alloc_error_handler]` item."] has_alloc_error_handler : bool , # [doc = " Names that were used to load the crates via `extern crate` or paths."] resolved_externs : UnordMap < Symbol , CrateNum > , # [doc = " Unused externs of the crate"] unused_externs : Vec < Symbol > , used_extern_options : FxHashSet < Symbol > , }
/* FP:creader.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_IMPL_0040
/* FP:creader.rs-0080 */ impl std :: fmt :: Debug for CStore { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("CStore") . finish_non_exhaustive () } }
/* FP:creader.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_ENUM_0041
/* FP:creader.rs-0082 */ pub enum LoadedMacro { MacroDef { def : MacroDef , ident : Ident , attrs : Vec < hir :: Attribute > , span : Span , edition : Edition , } , ProcMacro (SyntaxExtension) , }
/* FP:creader.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_STRUCT_0042
/* FP:creader.rs-0084 */ pub (crate) struct Library { pub source : CrateSource , pub metadata : MetadataBlob , }
/* FP:creader.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_ENUM_0043
/* FP:creader.rs-0086 */ enum LoadResult { Previous (CrateNum) , Loaded (Library) , }
/* FP:creader.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_STRUCT_0044
/* FP:creader.rs-0088 */ # [doc = " A reference to `CrateMetadata` that can also give access to whole crate store when necessary."] # [derive (Clone , Copy)] pub (crate) struct CrateMetadataRef < 'a > { pub cdata : & 'a CrateMetadata , pub cstore : & 'a CStore , }
/* FP:creader.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_IMPL_0045
/* FP:creader.rs-0090 */ impl std :: ops :: Deref for CrateMetadataRef < '_ > { type Target = CrateMetadata ; fn deref (& self) -> & Self :: Target { self . cdata } }
/* FP:creader.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_STRUCT_0046
/* FP:creader.rs-0092 */ struct CrateDump < 'a > (& 'a CStore) ;
/* FP:creader.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_IMPL_0047
/* FP:creader.rs-0094 */ impl < 'a > std :: fmt :: Debug for CrateDump < 'a > { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { writeln ! (fmt , "resolved crates:") ? ; for (cnum , data) in self . 0 . iter_crate_data () { writeln ! (fmt , "  name: {}" , data . name ()) ? ; writeln ! (fmt , "  cnum: {cnum}") ? ; writeln ! (fmt , "  hash: {}" , data . hash ()) ? ; writeln ! (fmt , "  reqd: {:?}" , data . dep_kind ()) ? ; writeln ! (fmt , "  priv: {:?}" , data . is_private_dep ()) ? ; let CrateSource { dylib , rlib , rmeta , sdylib_interface } = data . source () ; if let Some (dylib) = dylib { writeln ! (fmt , "  dylib: {}" , dylib . 0 . display ()) ? ; } if let Some (rlib) = rlib { writeln ! (fmt , "   rlib: {}" , rlib . 0 . display ()) ? ; } if let Some (rmeta) = rmeta { writeln ! (fmt , "   rmeta: {}" , rmeta . 0 . display ()) ? ; } if let Some (sdylib_interface) = sdylib_interface { writeln ! (fmt , "   sdylib interface: {}" , sdylib_interface . 0 . display ()) ? ; } } Ok (()) } }
/* FP:creader.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_ENUM_0048
/* FP:creader.rs-0096 */ # [doc = " Reason that a crate is being sourced as a dependency."] # [derive (Clone , Copy)] enum CrateOrigin < 'a > { # [doc = " This crate was a dependency of another crate."] IndirectDependency { # [doc = " Where this dependency was included from."] dep_root : & 'a CratePaths , # [doc = " True if the parent is private, meaning the dependent should also be private."] parent_private : bool , # [doc = " Dependency info about this crate."] dep : & 'a CrateDep , } , # [doc = " Injected by `rustc`."] Injected , # [doc = " Provided by `extern crate foo` or as part of the extern prelude."] Extern , }
/* FP:creader.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_IMPL_0049
/* FP:creader.rs-0098 */ impl < 'a > CrateOrigin < 'a > { # [doc = " Return the dependency root, if any."] fn dep_root (& self) -> Option < & 'a CratePaths > { match self { CrateOrigin :: IndirectDependency { dep_root , .. } => Some (dep_root) , _ => None , } } # [doc = " Return dependency information, if any."] fn dep (& self) -> Option < & 'a CrateDep > { match self { CrateOrigin :: IndirectDependency { dep , .. } => Some (dep) , _ => None , } } # [doc = " `Some(true)` if the dependency is private or its parent is private, `Some(false)` if the"] # [doc = " dependency is not private, `None` if it could not be determined."] fn private_dep (& self) -> Option < bool > { match self { CrateOrigin :: IndirectDependency { parent_private , dep , .. } => { Some (dep . is_private || * parent_private) } _ => None , } } }
/* FP:creader.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_IMPL_0050
/* FP:creader.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_FN_0051
/* FP:creader.rs-0102 */ fn fn_spans (krate : & ast :: Crate , name : Symbol) -> Vec < Span > { struct Finder { name : Symbol , spans : Vec < Span > , } impl < 'ast > visit :: Visitor < 'ast > for Finder { fn visit_item (& mut self , item : & 'ast ast :: Item) { if let Some (ident) = item . kind . ident () && ident . name == self . name && attr :: contains_name (& item . attrs , sym :: rustc_std_internal_symbol) { self . spans . push (item . span) ; } visit :: walk_item (self , item) } } let mut f = Finder { name , spans : Vec :: new () } ; visit :: walk_crate (& mut f , krate) ; f . spans }
/* FP:creader.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_FN_0052
/* FP:creader.rs-0104 */ fn format_dlopen_err (e : & (dyn std :: error :: Error + 'static)) -> String { e . sources () . map (| e | format ! (": {e}")) . collect () }
/* FP:creader.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_FN_0053
/* FP:creader.rs-0106 */ fn attempt_load_dylib (path : & Path) -> Result < libloading :: Library , libloading :: Error > { # [cfg (target_os = "aix")] if let Some (ext) = path . extension () && ext . eq ("a") { let library_name = path . file_stem () . expect ("expect a library name") ; let mut archive_member = std :: ffi :: OsString :: from ("a(") ; archive_member . push (library_name) ; archive_member . push (".so)") ; let new_path = path . with_extension (archive_member) ; let flags = libc :: RTLD_LAZY | libc :: RTLD_LOCAL | libc :: RTLD_MEMBER ; return unsafe { libloading :: os :: unix :: Library :: open (Some (& new_path) , flags) } . map (| lib | lib . into ()) ; } unsafe { libloading :: Library :: new (& path) } }
/* FP:creader.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_FN_0054
/* FP:creader.rs-0108 */ fn load_dylib (path : & Path , max_attempts : usize) -> Result < libloading :: Library , String > { assert ! (max_attempts > 0) ; let mut last_error = None ; for attempt in 0 .. max_attempts { debug ! ("Attempt to load proc-macro `{}`." , path . display ()) ; match attempt_load_dylib (path) { Ok (lib) => { if attempt > 0 { debug ! ("Loaded proc-macro `{}` after {} attempts." , path . display () , attempt + 1) ; } return Ok (lib) ; } Err (err) => { if ! matches ! (err , libloading :: Error :: LoadLibraryExW { .. }) { debug ! ("Failed to load proc-macro `{}`. Not retrying" , path . display ()) ; let err = format_dlopen_err (& err) ; if let Some (err) = err . strip_prefix (& format ! (": {}" , path . display ())) { return Err (err . to_string ()) ; } return Err (err) ; } last_error = Some (err) ; std :: thread :: sleep (Duration :: from_millis (100)) ; debug ! ("Failed to load proc-macro `{}`. Retrying." , path . display ()) ; } } } debug ! ("Failed to load proc-macro `{}` even after {} attempts." , path . display () , max_attempts) ; let last_error = last_error . unwrap () ; let message = if let Some (src) = last_error . source () { format ! ("{} ({src}) (retried {max_attempts} times)" , format_dlopen_err (& last_error)) } else { format ! ("{} (retried {max_attempts} times)" , format_dlopen_err (& last_error)) } ; Err (message) }
/* FP:creader.rs-0109 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_ENUM_0055
/* FP:creader.rs-0110 */ pub enum DylibError { DlOpen (String , String) , DlSym (String , String) , }
/* FP:creader.rs-0111 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_IMPL_0056
/* FP:creader.rs-0112 */ impl From < DylibError > for CrateError { fn from (err : DylibError) -> CrateError { match err { DylibError :: DlOpen (path , err) => CrateError :: DlOpen (path , err) , DylibError :: DlSym (path , err) => CrateError :: DlSym (path , err) , } } }
/* FP:creader.rs-0113 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_creader_FN_0057
/* FP:creader.rs-0114 */ pub unsafe fn load_symbol_from_dylib < T : Copy > (path : & Path , sym_name : & str ,) -> Result < T , DylibError > { let path = try_canonicalize (path) . unwrap () ; let lib = load_dylib (& path , 5) . map_err (| err | DylibError :: DlOpen (path . display () . to_string () , err)) ? ; let sym = unsafe { lib . get :: < T > (sym_name . as_bytes ()) } . map_err (| err | DylibError :: DlSym (path . display () . to_string () , format_dlopen_err (& err))) ? ; let sym = unsafe { sym . into_raw () } ; std :: mem :: forget (lib) ; Ok (* sym) }