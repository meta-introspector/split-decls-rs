/* FP:asm.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_asm_USE_0001
/* FP:asm.rs-0002 */ use crate :: rustc_complete :: { InlineAsmOptions , InlineAsmTemplatePiece } ;
/* FP:asm.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_asm_USE_0002
/* FP:asm.rs-0004 */ use crate :: rustc_complete :: def_id :: DefId ;
/* FP:asm.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_asm_USE_0003
/* FP:asm.rs-0006 */ use crate :: rustc_complete :: ty :: Instance ;
/* FP:asm.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_asm_USE_0004
/* FP:asm.rs-0008 */ use crate :: rustc_complete :: Span ;
/* FP:asm.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_asm_USE_0005
/* FP:asm.rs-0010 */ use crate :: rustc_target :: asm :: InlineAsmRegOrRegClass ;
/* FP:asm.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_asm_USE_0006
/* FP:asm.rs-0012 */ use super :: BackendTypes ;
/* FP:asm.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_asm_USE_0007
/* FP:asm.rs-0014 */ use crate :: mir :: operand :: OperandRef ;
/* FP:asm.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_asm_USE_0008
/* FP:asm.rs-0016 */ use crate :: mir :: place :: PlaceRef ;
/* FP:asm.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_asm_ENUM_0009
/* FP:asm.rs-0018 */ # [derive (Debug)] pub enum InlineAsmOperandRef < 'tcx , B : BackendTypes + ? Sized > { In { reg : InlineAsmRegOrRegClass , value : OperandRef < 'tcx , B :: Value > , } , Out { reg : InlineAsmRegOrRegClass , late : bool , place : Option < PlaceRef < 'tcx , B :: Value > > , } , InOut { reg : InlineAsmRegOrRegClass , late : bool , in_value : OperandRef < 'tcx , B :: Value > , out_place : Option < PlaceRef < 'tcx , B :: Value > > , } , Const { string : String , } , SymFn { instance : Instance < 'tcx > , } , SymStatic { def_id : DefId , } , Label { label : B :: BasicBlock , } , }
/* FP:asm.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_asm_ENUM_0010
/* FP:asm.rs-0020 */ # [derive (Debug)] pub enum GlobalAsmOperandRef < 'tcx > { Const { string : String } , SymFn { instance : Instance < 'tcx > } , SymStatic { def_id : DefId } , }
/* FP:asm.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_asm_TRAIT_0011
/* FP:asm.rs-0022 */ pub trait AsmBuilderMethods < 'tcx > : BackendTypes { # [doc = " Take an inline assembly expression and splat it out via LLVM"] fn codegen_inline_asm (& mut self , template : & [InlineAsmTemplatePiece] , operands : & [InlineAsmOperandRef < 'tcx , Self >] , options : InlineAsmOptions , line_spans : & [Span] , instance : Instance < '_ > , dest : Option < Self :: BasicBlock > , catch_funclet : Option < (Self :: BasicBlock , Option < & Self :: Funclet >) > ,) ; }
/* FP:asm.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_asm_TRAIT_0012
/* FP:asm.rs-0024 */ pub trait AsmCodegenMethods < 'tcx > { fn codegen_global_asm (& mut self , template : & [InlineAsmTemplatePiece] , operands : & [GlobalAsmOperandRef < 'tcx >] , options : InlineAsmOptions , line_spans : & [Span] ,) ; # [doc = " The mangled name of this instance"] # [doc = ""] # [doc = " Additional mangling is used on"] # [doc = " some targets to add a leading underscore (Mach-O)"] # [doc = " or byte count suffixes (x86 Windows)."] fn mangled_name (& self , instance : Instance < 'tcx >) -> String ; }