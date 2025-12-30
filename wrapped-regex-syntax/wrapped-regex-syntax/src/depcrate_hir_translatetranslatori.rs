// Generated macro for TranslatorI (struct)
macro_rules! Depcrate_hir_translateTranslatorI {
() => {
// Module: crate::hir::translate
// Provides: {"TranslatorI"}
// Dependencies: {}
# [doc = " The internal implementation of a translator."] # [doc = ""] # [doc = " This type is responsible for carrying around the original pattern string,"] # [doc = " which is not tied to the internal state of a translator."] # [doc = ""] # [doc = " A TranslatorI exists for the time it takes to translate a single Ast."] # [derive (Clone , Debug)] struct TranslatorI < 't , 'p > { trans : & 't Translator , pattern : & 'p str , }
};
}
