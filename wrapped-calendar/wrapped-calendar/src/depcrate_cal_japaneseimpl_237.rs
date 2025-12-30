// Generated macro for impl_237 (impl)
macro_rules! Depcrate_cal_japaneseimpl_237 {
() => {
// Module: crate::cal::japanese
// Provides: {"impl_237"}
// Dependencies: {}
impl Japanese { # [doc = " Creates a new [`Japanese`] using only modern eras (post-meiji) from compiled data."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] pub const fn new () -> Self { Self { eras : DataPayload :: from_static_ref (crate :: provider :: Baked :: SINGLETON_CALENDAR_JAPANESE_MODERN_V1 ,) , } } icu_provider :: gen_buffer_data_constructors ! (() -> error : DataError , functions : [new : skip , try_new_with_buffer_provider , try_new_unstable , Self ,]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new)] pub fn try_new_unstable < D : DataProvider < CalendarJapaneseModernV1 > + ? Sized > (provider : & D ,) -> Result < Self , DataError > { Ok (Self { eras : provider . load (Default :: default ()) ? . payload , }) } }
};
}
