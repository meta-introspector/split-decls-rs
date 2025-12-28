macro_rules! deps {
    () => {
        TranslatorBuilder!();
    };
}

macro_rules! ParserBuilder {
    () => {
        deps!();
        # [doc = " A builder for a regular expression parser."] # [doc = ""] # [doc = " This builder permits modifying configuration options for the parser."] # [doc = ""] # [doc = " This type combines the builder options for both the [AST"] # [doc = " `ParserBuilder`](ast::parse::ParserBuilder) and the [HIR"] # [doc = " `TranslatorBuilder`](hir::translate::TranslatorBuilder)."] # [derive (Clone , Debug , Default)] pub struct ParserBuilder { ast : ast :: parse :: ParserBuilder , hir : hir :: translate :: TranslatorBuilder , }
    };
}

ParserBuilder!();