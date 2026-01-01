/* FP:weak_lang_items.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_weak_lang_items_USE_0001
/* FP:weak_lang_items.rs-0002 */ use crate :: rustc_complete :: { Symbol , sym } ;
/* FP:weak_lang_items.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_weak_lang_items_USE_0002
/* FP:weak_lang_items.rs-0004 */ use crate :: LangItem ;
/* FP:weak_lang_items.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_weak_lang_items_MACRO_0003
/* FP:weak_lang_items.rs-0006 */ macro_rules ! weak_lang_items { ($ ($ item : ident , $ sym : ident ;) *) => { pub static WEAK_LANG_ITEMS : & [LangItem] = & [$ (LangItem ::$ item ,) *] ; impl LangItem { pub fn is_weak (self) -> bool { matches ! (self , $ (LangItem ::$ item) |*) } pub fn link_name (self) -> Option < Symbol > { match self { $ (LangItem ::$ item => Some (sym ::$ sym) ,) * _ => None , } } } } }
/* FP:weak_lang_items.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_weak_lang_items_MACRO_0004
/* FP:weak_lang_items.rs-0008 */ weak_lang_items ! { PanicImpl , rust_begin_unwind ; EhPersonality , rust_eh_personality ; EhCatchTypeinfo , rust_eh_catch_typeinfo ; }