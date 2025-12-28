macro_rules! deps {
    () => {
        Ast!();
        Translator!();
    };
}

macro_rules! TranslatorI {
    () => {
        deps!();
        # [doc = " The internal implementation of a translator."] # [doc = ""] # [doc = " This type is responsible for carrying around the original pattern string,"] # [doc = " which is not tied to the internal state of a translator."] # [doc = ""] # [doc = " A TranslatorI exists for the time it takes to translate a single Ast."] # [derive (Clone , Debug)] struct TranslatorI < 't , 'p > { trans : & 't Translator , pattern : & 'p str , }
    };
}

TranslatorI!();