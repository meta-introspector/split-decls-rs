// Generated macro for write_page (function)
macro_rules! Depcrate_test_dashboardwrite_page {
() => {
// Module: crate::test_dashboard
// Provides: {"write_page"}
// Dependencies: {}
fn write_page < T : Template > (dir : & Path , name : & str , template : & T) -> anyhow :: Result < () > { let mut file = BufWriter :: new (File :: create (dir . join (name)) ?) ; Template :: write_into (template , & mut file) ? ; Ok (()) }
};
}
