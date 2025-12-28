macro_rules! deps {
    () => {
        Info!();
        LifetimeSyntaxCategory!();
    };
}

macro_rules! impl_366 {
    () => {
        deps!();
        impl < 'tcx > Info < 'tcx > { fn syntax_source (& self) -> (hir :: LifetimeSyntax , LifetimeSource) { (self . lifetime . syntax , self . lifetime . source) } fn lifetime_syntax_category (& self) -> Option < LifetimeSyntaxCategory > { LifetimeSyntaxCategory :: new (self . syntax_source ()) } fn lifetime_name (& self) -> & str { self . lifetime . ident . as_str () } fn is_static (& self) -> bool { self . lifetime . is_static () } # [doc = " When reporting a lifetime that is implicit, we expand the span"] # [doc = " to include the type. Otherwise we end up pointing at nothing,"] # [doc = " which is a bit confusing."] fn reporting_span (& self) -> Span { if self . lifetime . is_implicit () { self . type_span } else { self . lifetime . ident . span } } # [doc = " When removing an explicit lifetime from a reference,"] # [doc = " we want to remove the whitespace after the lifetime."] # [doc = ""] # [doc = " ```rust"] # [doc = " fn x(a: &'_ u8) {}"] # [doc = " ```"] # [doc = ""] # [doc = " Should become:"] # [doc = ""] # [doc = " ```rust"] # [doc = " fn x(a: &u8) {}"] # [doc = " ```"] fn removing_span (& self) -> Span { let mut span = self . suggestion ("'dummy") . 0 ; if let Some (referenced_type_span) = self . referenced_type_span { span = span . until (referenced_type_span) ; } span } fn suggestion (& self , lifetime_name : & str) -> (Span , String) { self . lifetime . suggestion (lifetime_name) } }
    };
}

impl_366!();