macro_rules! deps {
    () => {
        Flags!();
    };
}

macro_rules! TranslatorBuilder {
    () => {
        deps!();
        # [doc = " A builder for constructing an AST->HIR translator."] # [derive (Clone , Debug)] pub struct TranslatorBuilder { utf8 : bool , line_terminator : u8 , flags : Flags , }
    };
}

TranslatorBuilder!();