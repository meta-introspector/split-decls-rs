/* FP:pretty_printing.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_USE_0001
/* FP:pretty_printing.rs-0002 */ use std :: num :: NonZero ;
/* FP:pretty_printing.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_USE_0002
/* FP:pretty_printing.rs-0004 */ use crate :: rustc_abi :: Align ;
/* FP:pretty_printing.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_USE_0003
/* FP:pretty_printing.rs-0006 */ use crate :: rustc_complete :: token :: CommentKind ;
/* FP:pretty_printing.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_USE_0004
/* FP:pretty_printing.rs-0008 */ use crate :: rustc_complete :: { AttrStyle , IntTy , UintTy } ;
/* FP:pretty_printing.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_USE_0005
/* FP:pretty_printing.rs-0010 */ use rustc_ast_pretty :: pp :: Printer ;
/* FP:pretty_printing.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_USE_0006
/* FP:pretty_printing.rs-0012 */ use crate :: rustc_complete :: hygiene :: Transparency ;
/* FP:pretty_printing.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_USE_0007
/* FP:pretty_printing.rs-0014 */ use crate :: rustc_complete :: { ErrorGuaranteed , Ident , Span , Symbol } ;
/* FP:pretty_printing.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_USE_0008
/* FP:pretty_printing.rs-0016 */ use crate :: rustc_target :: spec :: SanitizerSet ;
/* FP:pretty_printing.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_USE_0009
/* FP:pretty_printing.rs-0018 */ use thin_vec :: ThinVec ;
/* FP:pretty_printing.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_USE_0010
/* FP:pretty_printing.rs-0020 */ use crate :: limit :: Limit ;
/* FP:pretty_printing.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_TRAIT_0011
/* FP:pretty_printing.rs-0022 */ # [doc = " This trait is used to print attributes in `rustc_hir_pretty`."] # [doc = ""] # [doc = " For structs and enums it can be derived using [`rustc_macros::PrintAttribute`]."] # [doc = " The output will look a lot like a `Debug` implementation, but fields of several types"] # [doc = " like [`Span`]s and empty tuples, are gracefully skipped so they don't clutter the"] # [doc = " representation much."] pub trait PrintAttribute { # [doc = " Whether or not this will render as something meaningful, or if it's skipped"] # [doc = " (which will force the containing struct to also skip printing a comma"] # [doc = " and the field name)."] fn should_render (& self) -> bool ; fn print_attribute (& self , p : & mut Printer) ; }
/* FP:pretty_printing.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_IMPL_0012
/* FP:pretty_printing.rs-0024 */ impl PrintAttribute for u128 { fn should_render (& self) -> bool { true } fn print_attribute (& self , p : & mut Printer) { p . word (self . to_string ()) } }
/* FP:pretty_printing.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_IMPL_0013
/* FP:pretty_printing.rs-0026 */ impl < T : PrintAttribute > PrintAttribute for & T { fn should_render (& self) -> bool { T :: should_render (self) } fn print_attribute (& self , p : & mut Printer) { T :: print_attribute (self , p) } }
/* FP:pretty_printing.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_IMPL_0014
/* FP:pretty_printing.rs-0028 */ impl < T : PrintAttribute > PrintAttribute for Option < T > { fn should_render (& self) -> bool { self . as_ref () . is_some_and (| x | x . should_render ()) } fn print_attribute (& self , p : & mut Printer) { if let Some (i) = self { T :: print_attribute (i , p) } } }
/* FP:pretty_printing.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_IMPL_0015
/* FP:pretty_printing.rs-0030 */ impl < T : PrintAttribute > PrintAttribute for ThinVec < T > { fn should_render (& self) -> bool { self . is_empty () || self [0] . should_render () } fn print_attribute (& self , p : & mut Printer) { let mut last_printed = false ; p . word ("[") ; for i in self { if last_printed { p . word_space (",") ; } i . print_attribute (p) ; last_printed = i . should_render () ; } p . word ("]") ; } }
/* FP:pretty_printing.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_MACRO_0016
/* FP:pretty_printing.rs-0032 */ macro_rules ! print_skip { ($ ($ t : ty) ,* $ (,) ?) => { $ (impl PrintAttribute for $ t { fn should_render (& self) -> bool { false } fn print_attribute (& self , _ : & mut Printer) { } }) * } ; }
/* FP:pretty_printing.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_MACRO_0017
/* FP:pretty_printing.rs-0034 */ macro_rules ! print_disp { ($ ($ t : ty) ,* $ (,) ?) => { $ (impl PrintAttribute for $ t { fn should_render (& self) -> bool { true } fn print_attribute (& self , p : & mut Printer) { p . word (format ! ("{}" , self)) ; } }) * } ; }
/* FP:pretty_printing.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_MACRO_0018
/* FP:pretty_printing.rs-0036 */ macro_rules ! print_debug { ($ ($ t : ty) ,* $ (,) ?) => { $ (impl PrintAttribute for $ t { fn should_render (& self) -> bool { true } fn print_attribute (& self , p : & mut Printer) { p . word (format ! ("{:?}" , self)) ; } }) * } ; }
/* FP:pretty_printing.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_MACRO_0019
/* FP:pretty_printing.rs-0038 */ macro_rules ! print_tup { (num_should_render $ ($ ts : ident) *) => { 0 $ (+ $ ts . should_render () as usize) * } ; () => { } ; ($ t : ident $ ($ ts : ident) *) => { # [allow (non_snake_case , unused)] impl <$ t : PrintAttribute , $ ($ ts : PrintAttribute) ,*> PrintAttribute for ($ t , $ ($ ts) ,*) { fn should_render (& self) -> bool { let ($ t , $ ($ ts) ,*) = self ; print_tup ! (num_should_render $ t $ ($ ts) *) != 0 } fn print_attribute (& self , p : & mut Printer) { let ($ t , $ ($ ts) ,*) = self ; let parens = print_tup ! (num_should_render $ t $ ($ ts) *) > 1 ; if parens { p . popen () ; } let mut printed_anything = $ t . should_render () ; $ t . print_attribute (p) ; $ (if $ ts . should_render () { if printed_anything { p . word_space (",") ; } printed_anything = true ; } $ ts . print_attribute (p) ;) * if parens { p . pclose () ; } } } print_tup ! ($ ($ ts) *) ; } ; }
/* FP:pretty_printing.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_MACRO_0020
/* FP:pretty_printing.rs-0040 */ print_tup ! (A B C D E F G H) ;
/* FP:pretty_printing.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_MACRO_0021
/* FP:pretty_printing.rs-0042 */ print_skip ! (Span , () , ErrorGuaranteed) ;
/* FP:pretty_printing.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_MACRO_0022
/* FP:pretty_printing.rs-0044 */ print_disp ! (u16 , bool , NonZero < u32 >, Limit) ;
/* FP:pretty_printing.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_attrs_pretty_printing_MACRO_0023
/* FP:pretty_printing.rs-0046 */ print_debug ! (Symbol , Ident , UintTy , IntTy , Align , AttrStyle , CommentKind , Transparency , SanitizerSet ,) ;