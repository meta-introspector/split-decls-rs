macro_rules! deps {
    () => {
        SubstitutionPart!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl SubstitutionPart { pub fn is_addition (& self , sm : & SourceMap) -> bool { ! self . snippet . is_empty () && ! self . replaces_meaningful_content (sm) } pub fn is_deletion (& self , sm : & SourceMap) -> bool { self . snippet . trim () . is_empty () && self . replaces_meaningful_content (sm) } pub fn is_replacement (& self , sm : & SourceMap) -> bool { ! self . snippet . is_empty () && self . replaces_meaningful_content (sm) } # [doc = " Whether this is a replacement that overwrites source with a snippet"] # [doc = " in a way that isn't a superset of the original string. For example,"] # [doc = " replacing \"abc\" with \"abcde\" is not destructive, but replacing it"] # [doc = " it with \"abx\" is, since the \"c\" character is lost."] pub fn is_destructive_replacement (& self , sm : & SourceMap) -> bool { self . is_replacement (sm) && ! sm . span_to_snippet (self . span) . is_ok_and (| snippet | as_substr (snippet . trim () , self . snippet . trim ()) . is_some ()) } fn replaces_meaningful_content (& self , sm : & SourceMap) -> bool { sm . span_to_snippet (self . span) . map_or (! self . span . is_empty () , | snippet | ! snippet . trim () . is_empty ()) } # [doc = " Try to turn a replacement into an addition when the span that is being"] # [doc = " overwritten matches either the prefix or suffix of the replacement."] fn trim_trivial_replacements (& mut self , sm : & SourceMap) { if self . snippet . is_empty () { return ; } let Ok (snippet) = sm . span_to_snippet (self . span) else { return ; } ; if let Some ((prefix , substr , suffix)) = as_substr (& snippet , & self . snippet) { self . span = Span :: new (self . span . lo () + BytePos (prefix as u32) , self . span . hi () - BytePos (suffix as u32) , self . span . ctxt () , self . span . parent () ,) ; self . snippet = substr . to_string () ; } } }
    };
}

impl_31!()