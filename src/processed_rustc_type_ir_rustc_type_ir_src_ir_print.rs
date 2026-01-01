/* FP:ir_print.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_ir_print_USE_0001
/* FP:ir_print.rs-0002 */ use std :: fmt ;
/* FP:ir_print.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_ir_print_USE_0002
/* FP:ir_print.rs-0004 */ use crate :: { AliasTerm , AliasTy , Binder , ClosureKind , CoercePredicate , ExistentialProjection , ExistentialTraitRef , FnSig , HostEffectPredicate , Interner , NormalizesTo , OutlivesPredicate , PatternKind , ProjectionPredicate , SubtypePredicate , TraitPredicate , TraitRef , UnevaluatedConst , } ;
/* FP:ir_print.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_ir_print_TRAIT_0003
/* FP:ir_print.rs-0006 */ pub trait IrPrint < T > { fn print (t : & T , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result ; fn print_debug (t : & T , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result ; }
/* FP:ir_print.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_ir_print_MACRO_0004
/* FP:ir_print.rs-0008 */ macro_rules ! define_display_via_print { ($ ($ ty : ident) ,+ $ (,) ?) => { $ (impl < I : Interner > fmt :: Display for $ ty < I > { fn fmt (& self , fmt : & mut fmt :: Formatter <'_ >) -> fmt :: Result { < I as IrPrint <$ ty < I >>>:: print (self , fmt) } }) * } }
/* FP:ir_print.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_ir_print_IMPL_0005
/* FP:ir_print.rs-0010 */ impl < I : Interner , T > fmt :: Display for Binder < I , T > where I : IrPrint < Binder < I , T > > , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < I as IrPrint < Binder < I , T > > > :: print (self , fmt) } }
/* FP:ir_print.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_ir_print_MACRO_0006
/* FP:ir_print.rs-0012 */ macro_rules ! define_debug_via_print { ($ ($ ty : ident) ,+ $ (,) ?) => { $ (impl < I : Interner > fmt :: Debug for $ ty < I > { fn fmt (& self , fmt : & mut fmt :: Formatter <'_ >) -> fmt :: Result { < I as IrPrint <$ ty < I >>>:: print_debug (self , fmt) } }) * } }
/* FP:ir_print.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_ir_print_MACRO_0007
/* FP:ir_print.rs-0014 */ define_display_via_print ! (TraitRef , TraitPredicate , ExistentialTraitRef , ExistentialProjection , ProjectionPredicate , NormalizesTo , SubtypePredicate , CoercePredicate , HostEffectPredicate , AliasTy , AliasTerm , FnSig , PatternKind ,) ;
/* FP:ir_print.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_ir_print_MACRO_0008
/* FP:ir_print.rs-0016 */ define_debug_via_print ! (TraitRef , ExistentialTraitRef , PatternKind) ;
/* FP:ir_print.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_ir_print_IMPL_0009
/* FP:ir_print.rs-0018 */ impl < I : Interner , T > fmt :: Display for OutlivesPredicate < I , T > where I : IrPrint < OutlivesPredicate < I , T > > , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < I as IrPrint < OutlivesPredicate < I , T > > > :: print (self , fmt) } }
/* FP:ir_print.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_ir_print_MOD_0010