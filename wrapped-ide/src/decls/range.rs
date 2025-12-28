macro_rules! deps {
    () => {
        Analysis!();
        AnalysisHost!();
    };
}

macro_rules! range {
    () => {
        deps!();
        # [doc = " Creates analysis for a single file, returns range marked with a pair of $0."] pub (crate) fn range (# [rust_analyzer :: rust_fixture] ra_fixture : & str) -> (Analysis , FileRange) { let mut host = AnalysisHost :: default () ; let change_fixture = ChangeFixture :: parse (& host . db , ra_fixture) ; host . db . enable_proc_attr_macros () ; host . db . apply_change (change_fixture . change) ; let (file_id , range_or_offset) = change_fixture . file_position . expect ("expected a marker ($0)") ; let range = range_or_offset . expect_range () ; (host . analysis () , FileRange { file_id : file_id . file_id (& host . db) , range }) }
    };
}

range!()