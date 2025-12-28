macro_rules! deps {
    () => {
        PropertyUnicodeSet!();
    };
}

macro_rules! EmojiSet {
    () => {
        deps!();
        # [doc = " An Emoji set as defined by [`Unicode Technical Standard #51`](https://unicode.org/reports/tr51/#Emoji_Sets>)."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚫 This trait is sealed; it cannot be implemented by user code. If an API requests an item that implements this"] # [doc = " trait, please consider using a type from the implementors listed below."] # [doc = " </div>"] pub trait EmojiSet : crate :: private :: Sealed { # [doc (hidden)] type DataMarker : DataMarker < DataStruct = PropertyUnicodeSet < 'static > > ; # [doc (hidden)] # [cfg (feature = "compiled_data")] const SINGLETON : & 'static PropertyUnicodeSet < 'static > ; }
    };
}

EmojiSet!();