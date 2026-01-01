/* FP:upcast.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_upcast_TRAIT_0001
/* FP:upcast.rs-0002 */ # [doc = " An `Into`-like trait that takes `TyCtxt` to perform interner-specific transformations."] pub trait Upcast < I , T > { fn upcast (self , interner : I) -> T ; }
/* FP:upcast.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_upcast_IMPL_0002
/* FP:upcast.rs-0004 */ impl < I , T , U > Upcast < I , U > for T where U : UpcastFrom < I , T > , { fn upcast (self , interner : I) -> U { U :: upcast_from (self , interner) } }
/* FP:upcast.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_upcast_TRAIT_0003
/* FP:upcast.rs-0006 */ # [doc = " A `From`-like trait that takes `TyCtxt` to perform interner-specific transformations."] pub trait UpcastFrom < I , T > { fn upcast_from (from : T , interner : I) -> Self ; }
/* FP:upcast.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_upcast_IMPL_0004
/* FP:upcast.rs-0008 */ impl < I , T > UpcastFrom < I , T > for T { fn upcast_from (from : T , _tcx : I) -> Self { from } }