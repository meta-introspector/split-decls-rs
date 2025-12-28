macro_rules! mock {
    () => {
        # [proc_macro] pub fn mock (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { do_mock (input . into ()) . into () }
    };
}

mock!()