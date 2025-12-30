// Generated macro for impl_238 (impl)
macro_rules! Depcrate_cal_japaneseimpl_238 {
() => {
// Module: crate::cal::japanese
// Provides: {"impl_238"}
// Dependencies: {}
impl JapaneseExtended { # [doc = " Creates a new [`Japanese`] from using all eras (including pre-meiji) from compiled data."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] pub const fn new () -> Self { Self (Japanese { eras : DataPayload :: from_static_ref (crate :: provider :: Baked :: SINGLETON_CALENDAR_JAPANESE_EXTENDED_V1 ,) , }) } icu_provider :: gen_buffer_data_constructors ! (() -> error : DataError , functions : [new : skip , try_new_with_buffer_provider , try_new_unstable , Self ,]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new)] pub fn try_new_unstable < D : DataProvider < CalendarJapaneseExtendedV1 > + ? Sized > (provider : & D ,) -> Result < Self , DataError > { Ok (Self (Japanese { eras : provider . load (Default :: default ()) ? . payload . cast () , })) } }
};
}
