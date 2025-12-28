macro_rules! deps {
    () => {
        TakesNoArguments!();
    };
}

macro_rules! check_zero_tts {
    () => {
        deps!();
        # [doc = " Non-fatally assert that `tts` is empty. Note that this function"] # [doc = " returns even when `tts` is non-empty, macros that *need* to stop"] # [doc = " compilation should call `cx.diagnostic().abort_if_errors()`"] # [doc = " (this should be done as rarely as possible)."] pub (crate) fn check_zero_tts (cx : & ExtCtxt < '_ > , span : Span , tts : TokenStream , name : & str) { if ! tts . is_empty () { cx . dcx () . emit_err (errors :: TakesNoArguments { span , name }) ; } }
    };
}

check_zero_tts!()