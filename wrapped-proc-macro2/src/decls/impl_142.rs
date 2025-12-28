macro_rules! deps {
    () => {
        Group!();
        Span!();
        DelimSpan!();
        DelimSpanEnum!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl DelimSpan { pub (crate) fn new (group : & imp :: Group) -> Self { # [cfg (wrap_proc_macro)] let inner = match group { imp :: Group :: Compiler (group) => DelimSpanEnum :: Compiler { join : group . span () , open : group . span_open () , close : group . span_close () , } , imp :: Group :: Fallback (group) => DelimSpanEnum :: Fallback (group . span ()) , } ; # [cfg (not (wrap_proc_macro))] let inner = DelimSpanEnum :: Fallback (group . span ()) ; DelimSpan { inner , _marker : MARKER , } } # [doc = " Returns a span covering the entire delimited group."] pub fn join (& self) -> Span { match & self . inner { # [cfg (wrap_proc_macro)] DelimSpanEnum :: Compiler { join , .. } => Span :: _new (imp :: Span :: Compiler (* join)) , DelimSpanEnum :: Fallback (span) => Span :: _new_fallback (* span) , } } # [doc = " Returns a span for the opening punctuation of the group only."] pub fn open (& self) -> Span { match & self . inner { # [cfg (wrap_proc_macro)] DelimSpanEnum :: Compiler { open , .. } => Span :: _new (imp :: Span :: Compiler (* open)) , DelimSpanEnum :: Fallback (span) => Span :: _new_fallback (span . first_byte ()) , } } # [doc = " Returns a span for the closing punctuation of the group only."] pub fn close (& self) -> Span { match & self . inner { # [cfg (wrap_proc_macro)] DelimSpanEnum :: Compiler { close , .. } => Span :: _new (imp :: Span :: Compiler (* close)) , DelimSpanEnum :: Fallback (span) => Span :: _new_fallback (span . last_byte ()) , } } }
    };
}

impl_142!();