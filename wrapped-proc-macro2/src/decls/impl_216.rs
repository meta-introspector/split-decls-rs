macro_rules! deps {
    () => {
        Span!();
        Delimiter!();
        TokenStream!();
        Group!();
        DelimSpan!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl Group { fn _new (inner : imp :: Group) -> Self { Group { inner } } fn _new_fallback (inner : fallback :: Group) -> Self { Group { inner : imp :: Group :: from (inner) , } } # [doc = " Creates a new `Group` with the given delimiter and token stream."] # [doc = ""] # [doc = " This constructor will set the span for this group to"] # [doc = " `Span::call_site()`. To change the span you can use the `set_span`"] # [doc = " method below."] pub fn new (delimiter : Delimiter , stream : TokenStream) -> Self { Group { inner : imp :: Group :: new (delimiter , stream . inner) , } } # [doc = " Returns the punctuation used as the delimiter for this group: a set of"] # [doc = " parentheses, square brackets, or curly braces."] pub fn delimiter (& self) -> Delimiter { self . inner . delimiter () } # [doc = " Returns the `TokenStream` of tokens that are delimited in this `Group`."] # [doc = ""] # [doc = " Note that the returned token stream does not include the delimiter"] # [doc = " returned above."] pub fn stream (& self) -> TokenStream { TokenStream :: _new (self . inner . stream ()) } # [doc = " Returns the span for the delimiters of this token stream, spanning the"] # [doc = " entire `Group`."] # [doc = ""] # [doc = " ```text"] # [doc = " pub fn span(&self) -> Span {"] # [doc = "            ^^^^^^^"] # [doc = " ```"] pub fn span (& self) -> Span { Span :: _new (self . inner . span ()) } # [doc = " Returns the span pointing to the opening delimiter of this group."] # [doc = ""] # [doc = " ```text"] # [doc = " pub fn span_open(&self) -> Span {"] # [doc = "                 ^"] # [doc = " ```"] pub fn span_open (& self) -> Span { Span :: _new (self . inner . span_open ()) } # [doc = " Returns the span pointing to the closing delimiter of this group."] # [doc = ""] # [doc = " ```text"] # [doc = " pub fn span_close(&self) -> Span {"] # [doc = "                        ^"] # [doc = " ```"] pub fn span_close (& self) -> Span { Span :: _new (self . inner . span_close ()) } # [doc = " Returns an object that holds this group's `span_open()` and"] # [doc = " `span_close()` together (in a more compact representation than holding"] # [doc = " those 2 spans individually)."] pub fn delim_span (& self) -> DelimSpan { DelimSpan :: new (& self . inner) } # [doc = " Configures the span for this `Group`'s delimiters, but not its internal"] # [doc = " tokens."] # [doc = ""] # [doc = " This method will **not** set the span of all the internal tokens spanned"] # [doc = " by this group, but rather it will only set the span of the delimiter"] # [doc = " tokens at the level of the `Group`."] pub fn set_span (& mut self , span : Span) { self . inner . set_span (span . inner) ; } }
    };
}

impl_216!();