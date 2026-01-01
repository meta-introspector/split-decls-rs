/* FP:attr_wrapper.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_USE_0001
/* FP:attr_wrapper.rs-0002 */ use std :: borrow :: Cow ;
/* FP:attr_wrapper.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_USE_0002
/* FP:attr_wrapper.rs-0004 */ use std :: mem ;
/* FP:attr_wrapper.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_USE_0003
/* FP:attr_wrapper.rs-0006 */ use crate :: rustc_complete :: token :: Token ;
/* FP:attr_wrapper.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_USE_0004
/* FP:attr_wrapper.rs-0008 */ use crate :: rustc_complete :: tokenstream :: { AttrsTarget , LazyAttrTokenStream , NodeRange , ParserRange , Spacing , TokenCursor , } ;
/* FP:attr_wrapper.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_USE_0005
/* FP:attr_wrapper.rs-0010 */ use crate :: rustc_complete :: { self as ast , AttrVec , Attribute , HasAttrs , HasTokens } ;
/* FP:attr_wrapper.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_USE_0006
/* FP:attr_wrapper.rs-0012 */ use crate :: rustc_data_structures :: fx :: FxHashSet ;
/* FP:attr_wrapper.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_USE_0007
/* FP:attr_wrapper.rs-0014 */ use crate :: rustc_complete :: PResult ;
/* FP:attr_wrapper.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_USE_0008
/* FP:attr_wrapper.rs-0016 */ use crate :: rustc_complete :: parse :: ParseSess ;
/* FP:attr_wrapper.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_USE_0009
/* FP:attr_wrapper.rs-0018 */ use crate :: rustc_complete :: { DUMMY_SP , sym } ;
/* FP:attr_wrapper.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_USE_0010
/* FP:attr_wrapper.rs-0020 */ use thin_vec :: ThinVec ;
/* FP:attr_wrapper.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_USE_0011
/* FP:attr_wrapper.rs-0022 */ use super :: { Capturing , ForceCollect , Parser , Trailing } ;
/* FP:attr_wrapper.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_STRUCT_0012
/* FP:attr_wrapper.rs-0024 */ # [derive (Clone , Debug)] pub (super) struct CollectPos { start_token : (Token , Spacing) , cursor_snapshot : TokenCursor , start_pos : u32 , }
/* FP:attr_wrapper.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_ENUM_0013
/* FP:attr_wrapper.rs-0026 */ pub (super) enum UsePreAttrPos { No , Yes , }
/* FP:attr_wrapper.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_STRUCT_0014
/* FP:attr_wrapper.rs-0028 */ # [doc = " A wrapper type to ensure that the parser handles outer attributes correctly."] # [doc = " When we parse outer attributes, we need to ensure that we capture tokens"] # [doc = " for the attribute target. This allows us to perform cfg-expansion on"] # [doc = " a token stream before we invoke a derive proc-macro."] # [doc = ""] # [doc = " This wrapper prevents direct access to the underlying `ast::AttrVec`."] # [doc = " Parsing code can only get access to the underlying attributes"] # [doc = " by passing an `AttrWrapper` to `collect_tokens`."] # [doc = " This makes it difficult to accidentally construct an AST node"] # [doc = " (which stores an `ast::AttrVec`) without first collecting tokens."] # [doc = ""] # [doc = " This struct has its own module, to ensure that the parser code"] # [doc = " cannot directly access the `attrs` field."] # [derive (Debug , Clone)] pub (super) struct AttrWrapper { attrs : AttrVec , start_pos : Option < u32 > , }
/* FP:attr_wrapper.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_IMPL_0015
/* FP:attr_wrapper.rs-0030 */ impl AttrWrapper { pub (super) fn new (attrs : AttrVec , start_pos : u32) -> AttrWrapper { AttrWrapper { attrs , start_pos : Some (start_pos) } } pub (super) fn empty () -> AttrWrapper { AttrWrapper { attrs : AttrVec :: new () , start_pos : None } } pub (super) fn take_for_recovery (self , psess : & ParseSess) -> AttrVec { psess . dcx () . span_delayed_bug (self . attrs . get (0) . map (| attr | attr . span) . unwrap_or (DUMMY_SP) , "AttrVec is taken for recovery but no error is produced" ,) ; self . attrs } # [doc = " Prepend `self.attrs` to `attrs`."] pub (super) fn prepend_to_nt_inner (mut self , attrs : & mut AttrVec) { mem :: swap (attrs , & mut self . attrs) ; attrs . extend (self . attrs) ; } pub (super) fn is_empty (& self) -> bool { self . attrs . is_empty () } }
/* FP:attr_wrapper.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_FN_0016
/* FP:attr_wrapper.rs-0032 */ # [doc = " Returns `true` if `attrs` contains a `cfg` or `cfg_attr` attribute"] fn has_cfg_or_cfg_attr (attrs : & [Attribute]) -> bool { attrs . iter () . any (| attr | { attr . ident () . is_some_and (| ident | ident . name == sym :: cfg || ident . name == sym :: cfg_attr) }) }
/* FP:attr_wrapper.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_IMPL_0017
/* FP:attr_wrapper.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_attr_wrapper_FN_0018
/* FP:attr_wrapper.rs-0036 */ # [doc = " Tokens are needed if:"] # [doc = " - any non-single-segment attributes (other than doc comments) are present,"] # [doc = "   e.g. `rustfmt::skip`; or"] # [doc = " - any `cfg_attr` attributes are present; or"] # [doc = " - any single-segment, non-builtin attributes are present, e.g. `derive`,"] # [doc = "   `test`, `global_allocator`."] fn needs_tokens (attrs : & [ast :: Attribute]) -> bool { attrs . iter () . any (| attr | match attr . ident () { None => ! attr . is_doc_comment () , Some (ident) => { ident . name == sym :: cfg_attr || ! crate :: rustc_feature :: is_builtin_attr_name (ident . name) } }) }