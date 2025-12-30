// Generated macro for other_28 (other)
macro_rules! Depcrate_generatedother_28 {
() => {
// Module: crate::generated
// Provides: {"other_28"}
// Dependencies: {}
extern "C" { # [doc = " Telephony sensor stream for Speech Metrics"] # [doc = ""] # [doc = ""] # [doc = " This stream stores information data about your voice during phone calls including:"] # [doc = ""] # [doc = " ```text"] # [doc = "    - Tenor, pitch, and cadence"] # [doc = "    - Metrics such as average words per minute and average pause length"] # [doc = " ```"] # [doc = ""] # [doc = " This steam does not store any raw audio nor any audio or data from other parties."] # [doc = ""] # [doc = " Fetches from this stream return objects of type"] # [doc = " `SRSpeechMetrics`"] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/sensorkit/srsensortelephonyspeechmetrics?language=objc)"] pub static SRSensorTelephonySpeechMetrics : & 'static SRSensor ; }
};
}
