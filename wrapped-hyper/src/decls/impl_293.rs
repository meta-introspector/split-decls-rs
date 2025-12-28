macro_rules! impl_293 {
    () => {
        impl hyper_request { pub (super) fn finalize_request (& mut self) { if let Some (headers) = self . 0 . extensions_mut () . remove :: < hyper_headers > () { * self . 0 . headers_mut () = headers . headers ; self . 0 . extensions_mut () . insert (headers . orig_casing) ; self . 0 . extensions_mut () . insert (headers . orig_order) ; } } }
    };
}

impl_293!()