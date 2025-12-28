macro_rules! deps {
    () => {
        PropertyCodePointMap!();
        CodePointMapDataBorrowed!();
    };
}

macro_rules! CodePointMapData {
    () => {
        deps!();
        # [doc = " A wrapper around code point map data."] # [doc = ""] # [doc = " It is returned by APIs that return Unicode"] # [doc = " property data in a map-like form, ex: enumerated property value data keyed"] # [doc = " by code point. Access its data via the borrowed version,"] # [doc = " [`CodePointMapDataBorrowed`]."] # [derive (Debug , Clone)] pub struct CodePointMapData < T : TrieValue > { data : DataPayload < ErasedMarker < PropertyCodePointMap < 'static , T > > > , }
    };
}

CodePointMapData!()