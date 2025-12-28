macro_rules! deps {
    () => {
        ArgParser!();
        PathParser!();
        MetaItemParser!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl < 'a > MetaItemParser < 'a > { pub fn span (& self) -> Span { if let Some (other) = self . args . span () { self . path . span () . with_hi (other . hi ()) } else { self . path . span () } } # [doc = " Gets just the path, without the args. Some examples:"] # [doc = ""] # [doc = " - `#[rustfmt::skip]`: `rustfmt::skip` is a path"] # [doc = " - `#[allow(clippy::complexity)]`: `clippy::complexity` is a path"] # [doc = " - `#[inline]`: `inline` is a single segment path"] pub fn path (& self) -> & PathParser < 'a > { & self . path } # [doc = " Gets just the args parser, without caring about the path."] pub fn args (& self) -> & ArgParser < 'a > { & self . args } # [doc = " Asserts that this MetaItem starts with a word, or single segment path."] # [doc = ""] # [doc = " Some examples:"] # [doc = " - `#[inline]`: `inline` is a word"] # [doc = " - `#[rustfmt::skip]`: `rustfmt::skip` is a path,"] # [doc = "   and not a word and should instead be parsed using [`path`](Self::path)"] pub fn word_is (& self , sym : Symbol) -> Option < & ArgParser < 'a > > { self . path () . word_is (sym) . then (| | self . args ()) } }
    };
}

impl_302!()