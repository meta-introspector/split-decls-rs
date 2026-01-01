/* FP:entry.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_USE_0001
/* FP:entry.rs-0002 */ use crate :: rustc_complete :: attr ;
/* FP:entry.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_USE_0002
/* FP:entry.rs-0004 */ use crate :: rustc_complete :: entry :: EntryPointType ;
/* FP:entry.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_USE_0003
/* FP:entry.rs-0006 */ use crate :: rustc_complete :: codes :: * ;
/* FP:entry.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_USE_0004
/* FP:entry.rs-0008 */ use crate :: rustc_complete :: def :: DefKind ;
/* FP:entry.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_USE_0005
/* FP:entry.rs-0010 */ use crate :: rustc_complete :: def_id :: { CRATE_DEF_ID , DefId , LOCAL_CRATE , LocalDefId } ;
/* FP:entry.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_USE_0006
/* FP:entry.rs-0012 */ use crate :: rustc_complete :: { CRATE_HIR_ID , ItemId , Node } ;
/* FP:entry.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_USE_0007
/* FP:entry.rs-0014 */ use crate :: rustc_complete :: query :: Providers ;
/* FP:entry.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_USE_0008
/* FP:entry.rs-0016 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:entry.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_USE_0009
/* FP:entry.rs-0018 */ use crate :: rustc_complete :: RemapFileNameExt ;
/* FP:entry.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_USE_0010
/* FP:entry.rs-0020 */ use crate :: rustc_complete :: config :: { CrateType , EntryFnType , RemapPathScopeComponents , sigpipe } ;
/* FP:entry.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_USE_0011
/* FP:entry.rs-0022 */ use crate :: rustc_complete :: { Span , Symbol , sym } ;
/* FP:entry.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_USE_0012
/* FP:entry.rs-0024 */ use crate :: errors :: { AttrOnlyInFunctions , ExternMain , MultipleRustcMain , NoMainErr } ;
/* FP:entry.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_STRUCT_0013
/* FP:entry.rs-0026 */ struct EntryContext < 'tcx > { tcx : TyCtxt < 'tcx > , # [doc = " The function has the `#[rustc_main]` attribute."] rustc_main_fn : Option < (LocalDefId , Span) > , # [doc = " The functions that one might think are `main` but aren't, e.g."] # [doc = " main functions not defined at the top level. For diagnostics."] non_main_fns : Vec < Span > , }
/* FP:entry.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_FN_0014
/* FP:entry.rs-0028 */ fn entry_fn (tcx : TyCtxt < '_ > , () : ()) -> Option < (DefId , EntryFnType) > { let any_exe = tcx . crate_types () . contains (& CrateType :: Executable) ; if ! any_exe { return None ; } if attr :: contains_name (tcx . hir_attrs (CRATE_HIR_ID) , sym :: no_main) { return None ; } let mut ctxt = EntryContext { tcx , rustc_main_fn : None , non_main_fns : Vec :: new () } ; for id in tcx . hir_free_items () { check_and_search_item (id , & mut ctxt) ; } configure_main (tcx , & ctxt) }
/* FP:entry.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_FN_0015
/* FP:entry.rs-0030 */ fn attr_span_by_symbol (ctxt : & EntryContext < '_ > , id : ItemId , sym : Symbol) -> Option < Span > { let attrs = ctxt . tcx . hir_attrs (id . hir_id ()) ; attr :: find_by_name (attrs , sym) . map (| attr | attr . span ()) }
/* FP:entry.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_FN_0016
/* FP:entry.rs-0032 */ fn check_and_search_item (id : ItemId , ctxt : & mut EntryContext < '_ >) { if ! matches ! (ctxt . tcx . def_kind (id . owner_id) , DefKind :: Fn) { for attr in [sym :: rustc_main] { if let Some (span) = attr_span_by_symbol (ctxt , id , attr) { ctxt . tcx . dcx () . emit_err (AttrOnlyInFunctions { span , attr }) ; } } return ; } let at_root = ctxt . tcx . opt_local_parent (id . owner_id . def_id) == Some (CRATE_DEF_ID) ; let attrs = ctxt . tcx . hir_attrs (id . hir_id ()) ; let entry_point_type = crate :: rustc_ast :: entry :: entry_point_type (attrs , at_root , ctxt . tcx . opt_item_name (id . owner_id . to_def_id ()) ,) ; match entry_point_type { EntryPointType :: None => { } EntryPointType :: MainNamed => { } EntryPointType :: OtherMain => { ctxt . non_main_fns . push (ctxt . tcx . def_span (id . owner_id)) ; } EntryPointType :: RustcMainAttr => { if ctxt . rustc_main_fn . is_none () { ctxt . rustc_main_fn = Some ((id . owner_id . def_id , ctxt . tcx . def_span (id . owner_id))) ; } else { ctxt . tcx . dcx () . emit_err (MultipleRustcMain { span : ctxt . tcx . def_span (id . owner_id . to_def_id ()) , first : ctxt . rustc_main_fn . unwrap () . 1 , additional : ctxt . tcx . def_span (id . owner_id . to_def_id ()) , }) ; } } } }
/* FP:entry.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_FN_0017
/* FP:entry.rs-0034 */ fn configure_main (tcx : TyCtxt < '_ > , visitor : & EntryContext < '_ >) -> Option < (DefId , EntryFnType) > { if let Some ((local_def_id , _)) = visitor . rustc_main_fn { let def_id = local_def_id . to_def_id () ; Some ((def_id , EntryFnType :: Main { sigpipe : sigpipe (tcx) })) } else { if let Some (main_def) = tcx . resolutions (()) . main_def && let Some (def_id) = main_def . opt_fn_def_id () { if let Some (def_id) = def_id . as_local () && matches ! (tcx . hir_node_by_def_id (def_id) , Node :: ForeignItem (_)) { tcx . dcx () . emit_err (ExternMain { span : tcx . def_span (def_id) }) ; return None ; } return Some ((def_id , EntryFnType :: Main { sigpipe : sigpipe (tcx) })) ; } no_main_err (tcx , visitor) ; None } }
/* FP:entry.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_FN_0018
/* FP:entry.rs-0036 */ fn sigpipe (tcx : TyCtxt < '_ >) -> u8 { match tcx . sess . opts . unstable_opts . on_broken_pipe { crate :: rustc_target :: spec :: OnBrokenPipe :: Default => sigpipe :: DEFAULT , crate :: rustc_target :: spec :: OnBrokenPipe :: Kill => sigpipe :: SIG_DFL , crate :: rustc_target :: spec :: OnBrokenPipe :: Error => sigpipe :: SIG_IGN , crate :: rustc_target :: spec :: OnBrokenPipe :: Inherit => sigpipe :: INHERIT , } }
/* FP:entry.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_FN_0019
/* FP:entry.rs-0038 */ fn no_main_err (tcx : TyCtxt < '_ > , visitor : & EntryContext < '_ >) { let sp = tcx . def_span (CRATE_DEF_ID) ; let mut has_filename = true ; let filename = tcx . sess . local_crate_source_file () . map (| src | src . for_scope (& tcx . sess , RemapPathScopeComponents :: DIAGNOSTICS) . to_path_buf ()) . unwrap_or_else (| | { has_filename = false ; Default :: default () }) ; let main_def_opt = tcx . resolutions (()) . main_def ; let code = E0601 ; let add_teach_note = tcx . sess . teach (code) ; let file_empty = tcx . sess . source_map () . lookup_line (sp . hi ()) . is_err () ; tcx . dcx () . emit_err (NoMainErr { sp , crate_name : tcx . crate_name (LOCAL_CRATE) , has_filename , filename , file_empty , non_main_fns : visitor . non_main_fns . clone () , main_def_opt , add_teach_note , }) ; }
/* FP:entry.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_entry_FN_0020
/* FP:entry.rs-0040 */ pub fn provide (providers : & mut Providers) { * providers = Providers { entry_fn , .. * providers } ; }