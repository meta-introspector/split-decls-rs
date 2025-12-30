// Generated macro for ExportDriver (struct)
macro_rules! DepcrateExportDriver {
() => {
// Module: crate
// Provides: {"ExportDriver"}
// Dependencies: {}
# [doc = " Configuration for a data export operation."] # [doc = ""] # [doc = " Note that this only configures *which data* is exported. The input provider, usually"] # [doc = " `SourceDataProvider`, might expose more options about the data itself."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use icu_provider_export::blob_exporter::*;"] # [doc = " use icu_provider_export::prelude::*;"] # [doc = " use icu_provider_source::SourceDataProvider;"] # [doc = ""] # [doc = " let provider = SourceDataProvider::new();"] # [doc = ""] # [doc = " ExportDriver::new("] # [doc = "     [DataLocaleFamily::FULL],"] # [doc = "     DeduplicationStrategy::None.into(),"] # [doc = "     LocaleFallbacker::try_new_unstable(&provider).unwrap(),"] # [doc = " )"] # [doc = " .with_markers([icu::list::provider::ListAndV1::INFO])"] # [doc = " .export("] # [doc = "     &provider,"] # [doc = "     BlobExporter::new_with_sink(Box::new(&mut Vec::new())),"] # [doc = " )"] # [doc = " .unwrap();"] # [doc = " ```"] # [derive (Clone)] pub struct ExportDriver { markers : Option < BTreeSet < DataMarkerInfo > > , requested_families : HashMap < DataLocale , DataLocaleFamilyAnnotations > , # [expect (clippy :: type_complexity)] attributes_filters : HashMap < String , Arc < Box < dyn Fn (& DataMarkerAttributes) -> bool + Send + Sync + 'static > > > , fallbacker : LocaleFallbacker , include_full : bool , deduplication_strategy : DeduplicationStrategy , }
};
}
