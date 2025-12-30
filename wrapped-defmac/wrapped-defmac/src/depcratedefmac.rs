// Generated macro for defmac (macro)
macro_rules! Depcratedefmac {
() => {
// Module: crate
// Provides: {"defmac"}
// Dependencies: {}
# [doc = " A macro to define lambda-like macros inline."] # [doc = ""] # [doc = " Syntax:"] # [doc = ""] # [doc = " `defmac!(` *name* [ *pattern* [, *pattern* ... ]] `=>` *expression* `)`"] # [doc = ""] # [doc = " *name* is the name of the new macro, followed by 0 or more patterns"] # [doc = " separated by comma. A pattern can be just an argument name like `x`"] # [doc = " or a pattern like `ref value`, `(x, y)` etc."] # [doc = ""] # [doc = " Supports arbitrary many arguments."] # [macro_export (local_inner_macros)] macro_rules ! defmac { (@ nest $ name : ident ($ dol : tt) => ([$ ($ arg : ident) *] $ ($ result_body : tt) +)) => { macro_rules ! $ name { ($ ($ dol $ arg : expr) , *) => { $ ($ result_body) + } } } ; (@ nest $ name : ident ($ dol : tt) => ([$ ($ arg : ident) *] $ ($ result_body : tt) +) $ p1 : pat $ (, $ p2 : pat) *) => { defmac ! { @ nest $ name ($ dol) => ([marg $ ($ arg) *] match { $ dol marg } { $ p1 => $ ($ result_body) + }) $ ($ p2) ,* } } ; (@ revpats [$ ($ args : tt) *] [$ ($ pr : pat) ,*]) => { defmac ! { @ nest $ ($ args) * $ ($ pr) ,* } } ; (@ revpats [$ ($ args : tt) *] [$ ($ pr : pat) ,*] $ p1 : pat $ (, $ p2 : pat) *) => { defmac ! { @ revpats [$ ($ args) *] [$ p1 $ (, $ pr) *] $ ($ p2) ,* } } ; ($ name : ident $ ($ p1 : pat) ,* => $ result : expr) => { defmac ! { @ revpats [$ name ($) => ([] $ result)] [] $ ($ p1) ,* } } ; }
};
}
