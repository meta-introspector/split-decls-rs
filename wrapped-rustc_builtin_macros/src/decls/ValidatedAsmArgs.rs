macro_rules! ValidatedAsmArgs {
    () => {
        # [doc = " Validated assembly arguments, ready for macro expansion."] struct ValidatedAsmArgs { pub templates : Vec < Box < ast :: Expr > > , pub operands : Vec < (ast :: InlineAsmOperand , Span) > , named_args : FxIndexMap < Symbol , usize > , reg_args : GrowableBitSet < usize > , pub clobber_abis : Vec < (Symbol , Span) > , options : ast :: InlineAsmOptions , pub options_spans : Vec < Span > , }
    };
}

ValidatedAsmArgs!();