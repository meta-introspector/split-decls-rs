/* FP:lints.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_lints_USE_0001
/* FP:lints.rs-0002 */ use crate :: rustc_data_structures :: fingerprint :: Fingerprint ;
/* FP:lints.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_lints_USE_0002
/* FP:lints.rs-0004 */ use rustc_macros :: HashStable_Generic ;
/* FP:lints.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_lints_USE_0003
/* FP:lints.rs-0006 */ use crate :: rustc_complete :: Span ;
/* FP:lints.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_lints_USE_0004
/* FP:lints.rs-0008 */ use crate :: { AttrPath , HirId , Target } ;
/* FP:lints.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_lints_STRUCT_0005
/* FP:lints.rs-0010 */ # [derive (Debug)] pub struct DelayedLints { pub lints : Box < [DelayedLint] > , pub opt_hash : Option < Fingerprint > , }
/* FP:lints.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_lints_ENUM_0006
/* FP:lints.rs-0012 */ # [doc = " During ast lowering, no lints can be emitted."] # [doc = " That is because lints attach to nodes either in the AST, or on the built HIR."] # [doc = " When attached to AST nodes, they're emitted just before building HIR,"] # [doc = " and then there's a gap where no lints can be emitted until HIR is done."] # [doc = " The variants in this enum represent lints that are temporarily stashed during"] # [doc = " AST lowering to be emitted once HIR is built."] # [derive (Clone , Debug , HashStable_Generic)] pub enum DelayedLint { AttributeParsing (AttributeLint < HirId >) , }
/* FP:lints.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_lints_STRUCT_0007
/* FP:lints.rs-0014 */ # [derive (Clone , Debug , HashStable_Generic)] pub struct AttributeLint < Id > { pub id : Id , pub span : Span , pub kind : AttributeLintKind , }
/* FP:lints.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_lints_ENUM_0008
/* FP:lints.rs-0016 */ # [derive (Clone , Debug , HashStable_Generic)] pub enum AttributeLintKind { UnusedDuplicate { this : Span , other : Span , warning : bool } , IllFormedAttributeInput { suggestions : Vec < String > } , EmptyAttribute { first_span : Span } , InvalidTarget { name : AttrPath , target : Target , applied : Vec < String > , only : & 'static str } , InvalidStyle { name : AttrPath , is_used_as_inner : bool , target : Target , target_span : Span } , }