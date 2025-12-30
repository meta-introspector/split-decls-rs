// Generated macro for impl_27 (impl)
macro_rules! Depcrate_closerimpl_27 {
() => {
// Module: crate::closer
// Provides: {"impl_27"}
// Dependencies: {}
impl CaseMapCloser < CaseMapper > { icu_provider :: gen_buffer_data_constructors ! (() -> error : DataError , functions : [new : skip , try_new_with_buffer_provider , try_new_unstable , Self ,]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new)] pub fn try_new_unstable < P > (provider : & P) -> Result < Self , DataError > where P : DataProvider < CaseMapV1 > + DataProvider < CaseMapUnfoldV1 > + ? Sized , { let cm = CaseMapper :: try_new_unstable (provider) ? ; let unfold = provider . load (Default :: default ()) ? . payload ; Ok (Self { cm , unfold }) } }
};
}
