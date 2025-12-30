// Generated macro for TzifTestFile (struct)
macro_rules! Depcrate_tz_testdataTzifTestFile {
() => {
// Module: crate::tz::testdata
// Provides: {"TzifTestFile"}
// Dependencies: {}
# [doc = " A single TZif datum."] # [doc = ""] # [doc = " It contains the name of the time zone and the raw bytes of the"] # [doc = " corresponding TZif file."] # [derive (Clone , Copy)] pub (crate) struct TzifTestFile { pub (crate) name : & 'static str , pub (crate) data : & 'static [u8] , }
};
}
