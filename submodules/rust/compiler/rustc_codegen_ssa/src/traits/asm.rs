mkuse!{use rustc_ast :: { InlineAsmOptions , InlineAsmTemplatePiece } ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_middle :: ty :: Instance ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_target :: asm :: InlineAsmRegOrRegClass ;}
mkuse!{use super :: BackendTypes ;}
mkuse!{use crate :: mir :: operand :: OperandRef ;}
mkuse!{use crate :: mir :: place :: PlaceRef ;}
mkitem!{mkenum!{# [derive (Debug)] pub enum InlineAsmOperandRef < 'tcx , B : BackendTypes + ? Sized > { In { reg : InlineAsmRegOrRegClass , value : OperandRef < 'tcx , B :: Value > , } , Out { reg : InlineAsmRegOrRegClass , late : bool , place : Option < PlaceRef < 'tcx , B :: Value > > , } , InOut { reg : InlineAsmRegOrRegClass , late : bool , in_value : OperandRef < 'tcx , B :: Value > , out_place : Option < PlaceRef < 'tcx , B :: Value > > , } , Const { string : String , } , SymFn { instance : Instance < 'tcx > , } , SymStatic { def_id : DefId , } , Label { label : B :: BasicBlock , } , }}}
mkitem!{mkenum!{# [derive (Debug)] pub enum GlobalAsmOperandRef < 'tcx > { Const { string : String } , SymFn { instance : Instance < 'tcx > } , SymStatic { def_id : DefId } , }}}
mkitem!{mktrait!{pub trait AsmBuilderMethods < 'tcx > : BackendTypes { # [doc = " Take an inline assembly expression and splat it out via LLVM"] fn codegen_inline_asm (& mut self , template : & [InlineAsmTemplatePiece] , operands : & [InlineAsmOperandRef < 'tcx , Self >] , options : InlineAsmOptions , line_spans : & [Span] , instance : Instance < '_ > , dest : Option < Self :: BasicBlock > , catch_funclet : Option < (Self :: BasicBlock , Option < & Self :: Funclet >) > ,) ; }}}
mkitem!{mktrait!{pub trait AsmCodegenMethods < 'tcx > { fn codegen_global_asm (& mut self , template : & [InlineAsmTemplatePiece] , operands : & [GlobalAsmOperandRef < 'tcx >] , options : InlineAsmOptions , line_spans : & [Span] ,) ; # [doc = " The mangled name of this instance"] # [doc = ""] # [doc = " Additional mangling is used on"] # [doc = " some targets to add a leading underscore (Mach-O)"] # [doc = " or byte count suffixes (x86 Windows)."] fn mangled_name (& self , instance : Instance < 'tcx >) -> String ; }}}