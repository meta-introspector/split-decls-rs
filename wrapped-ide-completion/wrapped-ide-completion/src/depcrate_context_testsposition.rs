// Generated macro for position (function)
macro_rules! Depcrate_context_testsposition {
() => {
// Module: crate::context::tests
// Provides: {"position"}
// Dependencies: {}
# [doc = " Creates analysis from a multi-file fixture, returns positions marked with $0."] pub (crate) fn position (# [rust_analyzer :: rust_fixture] ra_fixture : & str ,) -> (RootDatabase , FilePosition) { let mut database = RootDatabase :: default () ; let change_fixture = ChangeFixture :: parse (& database , ra_fixture) ; database . enable_proc_attr_macros () ; database . apply_change (change_fixture . change) ; let (file_id , range_or_offset) = change_fixture . file_position . expect ("expected a marker ($0)") ; let offset = range_or_offset . expect_offset () ; let position = FilePosition { file_id : file_id . file_id (& database) , offset } ; (database , position) }
};
}
