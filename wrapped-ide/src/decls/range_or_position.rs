macro_rules! deps {
    () => {
        Analysis!();
        AnalysisHost!();
    };
}

macro_rules! range_or_position {
    () => {
        deps!();
        # [doc = " Creates analysis for a single file, returns range marked with a pair of $0 or a position marked with $0."] pub (crate) fn range_or_position (# [rust_analyzer :: rust_fixture] ra_fixture : & str ,) -> (Analysis , FileId , RangeOrOffset) { let mut host = AnalysisHost :: default () ; let change_fixture = ChangeFixture :: parse (& host . db , ra_fixture) ; host . db . enable_proc_attr_macros () ; host . db . apply_change (change_fixture . change) ; let (file_id , range_or_offset) = change_fixture . file_position . expect ("expected a marker ($0)") ; (host . analysis () , file_id . file_id (& host . db) , range_or_offset) }
    };
}

range_or_position!();