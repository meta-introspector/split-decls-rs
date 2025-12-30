// Generated macro for impl_148 (impl)
macro_rules! Depcrate_titlecaseimpl_148 {
() => {
// Module: crate::titlecase
// Provides: {"impl_148"}
// Dependencies: {}
impl TitlecaseMapper < CaseMapper > { icu_provider :: gen_buffer_data_constructors ! (() -> error : DataError , functions : [new : skip , try_new_with_buffer_provider , try_new_unstable , Self ,]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new)] pub fn try_new_unstable < P > (provider : & P) -> Result < Self , DataError > where P : DataProvider < CaseMapV1 > + DataProvider < PropertyEnumGeneralCategoryV1 > + ? Sized , { let cm = CaseMapper :: try_new_unstable (provider) ? ; let gc = icu_properties :: CodePointMapData :: < icu_properties :: props :: GeneralCategory > :: try_new_unstable (provider) ? ; Ok (Self { cm , gc }) } }
};
}
