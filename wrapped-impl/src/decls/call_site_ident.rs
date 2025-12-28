macro_rules! call_site_ident {
    () => {
        pub (crate) fn call_site_ident (ident : & Ident) -> Ident { let mut ident = ident . clone () ; ident . set_span (ident . span () . resolved_at (Span :: call_site ())) ; ident }
    };
}

call_site_ident!();