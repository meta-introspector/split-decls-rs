/* FP:weak_lang_items.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_weak_lang_items_USE_0001
/* FP:weak_lang_items.rs-0002 */ use rustc_ast as ast ;
/* FP:weak_lang_items.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_weak_lang_items_USE_0002
/* FP:weak_lang_items.rs-0004 */ use crate :: rustc_complete :: visit ;
/* FP:weak_lang_items.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_weak_lang_items_USE_0003
/* FP:weak_lang_items.rs-0006 */ use crate :: rustc_data_structures :: fx :: FxHashSet ;
/* FP:weak_lang_items.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_weak_lang_items_USE_0004
/* FP:weak_lang_items.rs-0008 */ use crate :: rustc_complete :: lang_items :: { self , LangItem } ;
/* FP:weak_lang_items.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_weak_lang_items_USE_0005
/* FP:weak_lang_items.rs-0010 */ use crate :: rustc_complete :: weak_lang_items :: WEAK_LANG_ITEMS ;
/* FP:weak_lang_items.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_weak_lang_items_USE_0006
/* FP:weak_lang_items.rs-0012 */ use crate :: rustc_complete :: middle :: lang_items :: required ;
/* FP:weak_lang_items.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_weak_lang_items_USE_0007
/* FP:weak_lang_items.rs-0014 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:weak_lang_items.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_weak_lang_items_USE_0008
/* FP:weak_lang_items.rs-0016 */ use crate :: rustc_complete :: config :: CrateType ;
/* FP:weak_lang_items.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_weak_lang_items_USE_0009
/* FP:weak_lang_items.rs-0018 */ use crate :: errors :: { MissingLangItem , MissingPanicHandler , PanicUnwindWithoutStd , UnknownExternLangItem , } ;
/* FP:weak_lang_items.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_weak_lang_items_FN_0010
/* FP:weak_lang_items.rs-0020 */ # [doc = " Checks the crate for usage of weak lang items, returning a vector of all the"] # [doc = " lang items required by this crate, but not defined yet."] pub (crate) fn check_crate (tcx : TyCtxt < '_ > , items : & mut lang_items :: LanguageItems , krate : & ast :: Crate ,) { if items . eh_personality () . is_none () { items . missing . push (LangItem :: EhPersonality) ; } if tcx . sess . target . os == "emscripten" && items . eh_catch_typeinfo () . is_none () && ! tcx . sess . opts . unstable_opts . emscripten_wasm_eh { items . missing . push (LangItem :: EhCatchTypeinfo) ; } visit :: Visitor :: visit_crate (& mut WeakLangItemVisitor { tcx , items } , krate) ; verify (tcx , items) ; }
/* FP:weak_lang_items.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_weak_lang_items_STRUCT_0011
/* FP:weak_lang_items.rs-0022 */ struct WeakLangItemVisitor < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , items : & 'a mut lang_items :: LanguageItems , }
/* FP:weak_lang_items.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_weak_lang_items_IMPL_0012
/* FP:weak_lang_items.rs-0024 */ impl < 'ast > visit :: Visitor < 'ast > for WeakLangItemVisitor < '_ , '_ > { fn visit_foreign_item (& mut self , i : & 'ast ast :: ForeignItem) { if let Some ((lang_item , _)) = lang_items :: extract (& i . attrs) { if let Some (item) = LangItem :: from_name (lang_item) && item . is_weak () { if self . items . get (item) . is_none () { self . items . missing . push (item) ; } } else { self . tcx . dcx () . emit_err (UnknownExternLangItem { span : i . span , lang_item }) ; } } } }
/* FP:weak_lang_items.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_weak_lang_items_FN_0013
/* FP:weak_lang_items.rs-0026 */ fn verify (tcx : TyCtxt < '_ > , items : & lang_items :: LanguageItems) { let needs_check = tcx . crate_types () . iter () . any (| kind | match * kind { CrateType :: Dylib | CrateType :: ProcMacro | CrateType :: Cdylib | CrateType :: Executable | CrateType :: Staticlib | CrateType :: Sdylib => true , CrateType :: Rlib => false , }) ; if ! needs_check { return ; } let mut missing = FxHashSet :: default () ; for & cnum in tcx . crates (()) . iter () { for & item in tcx . missing_lang_items (cnum) . iter () { missing . insert (item) ; } } for & item in WEAK_LANG_ITEMS . iter () { if missing . contains (& item) && required (tcx , item) && items . get (item) . is_none () { if item == LangItem :: PanicImpl { tcx . dcx () . emit_err (MissingPanicHandler) ; } else if item == LangItem :: EhPersonality { tcx . dcx () . emit_err (PanicUnwindWithoutStd) ; } else { tcx . dcx () . emit_err (MissingLangItem { name : item . name () }) ; } } } }