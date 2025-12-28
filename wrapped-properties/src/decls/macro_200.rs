macro_rules! macro_200 {
    () => {
        make_binary_property ! { name : "NFKC_Inert" ; short_name : "NFKC_Inert" ; ident : NfkcInert ; data_marker : crate :: provider :: PropertyBinaryNfkcInertV1 ; singleton : SINGLETON_PROPERTY_BINARY_NFKC_INERT_V1 ; # [doc = " Characters that are inert under NFKC, i.e., they do not interact with adjacent characters."] }
    };
}

macro_200!()