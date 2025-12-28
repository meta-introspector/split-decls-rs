macro_rules! expand_panic {
    () => {
        # [doc = " This expands to either"] # [doc = " - `$crate::panic::panic_2015!(...)` or"] # [doc = " - `$crate::panic::panic_2021!(...)`"] # [doc = " depending on the edition."] # [doc = ""] # [doc = " This is used for both std::panic!() and core::panic!()."] # [doc = ""] # [doc = " `$crate` will refer to either the `std` or `core` crate depending on which"] # [doc = " one we're expanding from."] pub (crate) fn expand_panic < 'cx > (cx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { let mac = if use_panic_2021 (sp) { sym :: panic_2021 } else { sym :: panic_2015 } ; expand (mac , cx , sp , tts) }
    };
}

expand_panic!();