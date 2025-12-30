// Generated macro for impl_1058 (impl)
macro_rules! Depcrate_tz_testdataimpl_1058 {
() => {
// Module: crate::tz::testdata
// Provides: {"impl_1058"}
// Dependencies: {}
impl TzifTestFile { # [doc = " Look up the TZif test file for the given time zone name."] # [doc = ""] # [doc = " If one doesn't exist, then this panics and fails the current"] # [doc = " test."] pub (crate) fn get (name : & str) -> TzifTestFile { for & tzif_file in TZIF_TEST_FILES { if tzif_file . name == name { return tzif_file ; } } panic ! ("could not find TZif test file for {name:?}") } # [doc = " Parse this test TZif data into a structured representation."] # [cfg (not (miri))] pub (crate) fn parse (self) -> TzifOwned { use alloc :: string :: ToString ; let name = Some (self . name . to_string ()) ; TzifOwned :: parse (name , self . data) . unwrap_or_else (| err | { panic ! ("failed to parse TZif test file for {:?}: {err}" , self . name) }) } # [doc = " Parse this test TZif data as if it were V1."] # [cfg (not (miri))] pub (crate) fn parse_v1 (self) -> TzifOwned { use alloc :: string :: ToString ; let name = Some (self . name . to_string ()) ; let mut data = self . data . to_vec () ; data [4] = 0 ; TzifOwned :: parse (name , & data) . unwrap_or_else (| err | { panic ! ("failed to parse V1 TZif test file for {:?}: {err}" , self . name) }) } }
};
}
