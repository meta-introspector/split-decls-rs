macro_rules! macro_206 {
    () => {
        make_binary_property ! { name : "Quotation_Mark" ; short_name : "QMark" ; ident : QuotationMark ; data_marker : crate :: provider :: PropertyBinaryQuotationMarkV1 ; singleton : SINGLETON_PROPERTY_BINARY_QUOTATION_MARK_V1 ; # [doc = " Punctuation characters that function as quotation marks."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::QuotationMark;"] # [doc = ""] # [doc = " let quotation_mark = CodePointSetData::new::<QuotationMark>();"] # [doc = ""] # [doc = " assert!(quotation_mark.contains('\\''));"] # [doc = " assert!(quotation_mark.contains('„'));  // U+201E DOUBLE LOW-9 QUOTATION MARK"] # [doc = " assert!(!quotation_mark.contains('<'));"] # [doc = " ```"] }
    };
}

macro_206!()