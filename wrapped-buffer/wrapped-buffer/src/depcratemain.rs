// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let blob = std :: fs :: read (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/buffer_data.postcard" ,)) . expect ("pre-computed postcard buffer should exist") ; let provider = BlobDataProvider :: try_new_from_blob (blob . into_boxed_slice ()) . expect ("deserialization should succeed") ; let formatter = FixedCalendarDateTimeFormatter :: < Gregorian , _ > :: try_new_with_buffer_provider (& provider , locale ! ("my") . into () , YMDT :: medium () ,) . expect ("locale 'my' should be present in compiled data") ; let date = Date :: try_new_gregorian (2022 , 12 , 23) . expect ("constant should be valid datetime") ; let time = Time :: try_new (12 , 54 , 29 , 0) . unwrap () ; let result = formatter . format (& DateTime { date , time }) . to_string () ; assert_eq ! (result , "၂၀၂၂ ဒီ ၂၃ ၁၂:၅၄:၂၉") ; println ! ("{result}") ; }
};
}
