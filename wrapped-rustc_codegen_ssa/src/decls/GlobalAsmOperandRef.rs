macro_rules! GlobalAsmOperandRef {
    () => {
        # [derive (Debug)] pub enum GlobalAsmOperandRef < 'tcx > { Const { string : String } , SymFn { instance : Instance < 'tcx > } , SymStatic { def_id : DefId } , }
    };
}

GlobalAsmOperandRef!();