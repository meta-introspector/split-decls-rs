// Generated macro for tests (module)
macro_rules! Depcrate_formattests {
() => {
// Module: crate::format
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use icu_locale_core :: locale ; use writeable :: assert_writeable_eq ; use crate :: DecimalFormatter ; # [test] pub fn test_es_mx () { let locale = locale ! ("es-MX") . into () ; let fmt = DecimalFormatter :: try_new (locale , Default :: default ()) . unwrap () ; let fd = "12345.67" . parse () . unwrap () ; assert_writeable_eq ! (fmt . format (& fd) , "12,345.67") ; } }
};
}
