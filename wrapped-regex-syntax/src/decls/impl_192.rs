macro_rules! deps {
    () => {
        Hir!();
        Ast!();
        TranslatorBuilder!();
        Result!();
        Translator!();
        TranslatorI!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl Translator { # [doc = " Create a new translator using the default configuration."] pub fn new () -> Translator { TranslatorBuilder :: new () . build () } # [doc = " Translate the given abstract syntax tree (AST) into a high level"] # [doc = " intermediate representation (HIR)."] # [doc = ""] # [doc = " If there was a problem doing the translation, then an HIR-specific"] # [doc = " error is returned."] # [doc = ""] # [doc = " The original pattern string used to produce the `Ast` *must* also be"] # [doc = " provided. The translator does not use the pattern string during any"] # [doc = " correct translation, but is used for error reporting."] pub fn translate (& mut self , pattern : & str , ast : & Ast) -> Result < Hir > { ast :: visit (ast , TranslatorI :: new (self , pattern)) } }
    };
}

impl_192!();