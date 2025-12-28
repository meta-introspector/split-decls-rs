macro_rules! deps {
    () => {
        ComparisonKind!();
        CguReuse!();
    };
}

macro_rules! TrackerData {
    () => {
        deps!();
        struct TrackerData { actual_reuse : UnordMap < String , CguReuse > , expected_reuse : UnordMap < String , (String , Span , CguReuse , ComparisonKind) > , }
    };
}

TrackerData!()