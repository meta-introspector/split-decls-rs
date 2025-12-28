macro_rules! deps {
    () => {
        AsmCodegenMethods!();
    };
}

macro_rules! codegen_naked_asm {
    () => {
        deps!();
        pub fn codegen_naked_asm < 'a , 'tcx , Cx : LayoutOf < 'tcx , LayoutOfResult = TyAndLayout < 'tcx > > + FnAbiOf < 'tcx , FnAbiOfResult = & 'tcx FnAbi < 'tcx , Ty < 'tcx > > > + AsmCodegenMethods < 'tcx > , > (cx : & 'a mut Cx , instance : Instance < 'tcx > , item_data : MonoItemData ,) { assert ! (! instance . args . has_infer ()) ; let mir = cx . tcx () . instance_mir (instance . def) ; let rustc_middle :: mir :: TerminatorKind :: InlineAsm { asm_macro : _ , template , ref operands , options , line_spans , targets : _ , unwind : _ , } = mir . basic_blocks [START_BLOCK] . terminator () . kind else { bug ! ("#[naked] functions should always terminate with an asm! block") } ; let operands : Vec < _ > = operands . iter () . map (| op | inline_to_global_operand :: < Cx > (cx , instance , op)) . collect () ; let name = cx . mangled_name (instance) ; let fn_abi = cx . fn_abi_of_instance (instance , ty :: List :: empty ()) ; let (begin , end) = prefix_and_suffix (cx . tcx () , instance , & name , item_data , fn_abi) ; let mut template_vec = Vec :: new () ; template_vec . push (rustc_ast :: ast :: InlineAsmTemplatePiece :: String (begin . into ())) ; template_vec . extend (template . iter () . cloned ()) ; template_vec . push (rustc_ast :: ast :: InlineAsmTemplatePiece :: String (end . into ())) ; cx . codegen_global_asm (& template_vec , & operands , options , line_spans) ; }
    };
}

codegen_naked_asm!()