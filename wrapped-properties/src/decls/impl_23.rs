macro_rules! deps {
    () => {
        EmojiSetDataBorrowed!();
        EmojiSet!();
        EmojiSetData!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl EmojiSetDataBorrowed < 'static > { # [doc = " Creates a new [`EmojiSetDataBorrowed`] for a [`EmojiSet`]."] # [doc = ""] # [doc = " See the documentation on [`EmojiSet`] implementations for details."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [inline] # [cfg (feature = "compiled_data")] pub const fn new < P : EmojiSet > () -> Self { EmojiSetDataBorrowed { set : P :: SINGLETON } } # [doc = " Cheaply converts a [`EmojiSetDataBorrowed<'static>`] into a [`EmojiSetData`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`EmojiSetData`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`EmojiSetDataBorrowed`]."] pub const fn static_to_owned (self) -> EmojiSetData { EmojiSetData { data : DataPayload :: from_static_ref (self . set) , } } }
    };
}

impl_23!();