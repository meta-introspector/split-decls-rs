// Generated macro for EmptyDataProvider (struct)
macro_rules! Depcrate_emptyEmptyDataProvider {
() => {
// Module: crate::empty
// Provides: {"EmptyDataProvider"}
// Dependencies: {}
# [doc = " A data provider that always returns an error."] # [doc = ""] # [doc = " The returned error kind is configurable."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu_provider::hello_world::HelloWorldV1;"] # [doc = " use icu_provider::prelude::*;"] # [doc = " use icu_provider_adapters::empty::EmptyDataProvider;"] # [doc = ""] # [doc = " let provider = EmptyDataProvider::new();"] # [doc = ""] # [doc = " assert!(matches!("] # [doc = "     DataProvider::<HelloWorldV1>::load(&provider, Default::default()),"] # [doc = "     Err(DataError {"] # [doc = "         kind: DataErrorKind::MarkerNotFound,"] # [doc = "         .."] # [doc = "     })"] # [doc = " ));"] # [doc = " ```"] # [derive (Debug)] pub struct EmptyDataProvider { error_kind : DataErrorKind , }
};
}
