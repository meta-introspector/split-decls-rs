// Generated macro for FsDataProvider (struct)
macro_rules! Depcrate_fs_data_providerFsDataProvider {
() => {
// Module: crate::fs_data_provider
// Provides: {"FsDataProvider"}
// Dependencies: {}
# [doc = " A data provider that reads ICU4X data from a filesystem directory."] # [doc = ""] # [doc = " [`FsDataProvider`] implements [`BufferProvider`], so it can be used in"] # [doc = " `*_with_buffer_provider` constructors across ICU4X."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu_locale_core::locale;"] # [doc = " use icu_provider::hello_world::HelloWorldFormatter;"] # [doc = " use icu_provider_fs::FsDataProvider;"] # [doc = " use writeable::assert_writeable_eq;"] # [doc = ""] # [doc = " // Create a DataProvider from data files stored in a filesystem directory:"] # [doc = " let provider = FsDataProvider::try_new(\"tests/data/json\".into())"] # [doc = "     .expect(\"Directory exists\");"] # [doc = ""] # [doc = " // Check that it works:"] # [doc = " let formatter = HelloWorldFormatter::try_new_with_buffer_provider("] # [doc = "     &provider,"] # [doc = "     locale!(\"la\").into(),"] # [doc = " )"] # [doc = " .expect(\"locale exists\");"] # [doc = ""] # [doc = " assert_writeable_eq!(formatter.format(), \"Ave, munde\");"] # [doc = " ```"] # [derive (Debug , PartialEq , Clone)] pub struct FsDataProvider { root : PathBuf , manifest : Manifest , }
};
}
