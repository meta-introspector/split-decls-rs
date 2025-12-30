// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
# [doc = " Converts from a [`jiff::tz::Offset`] to a [`icu_time::zone::UtcOffset`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff_icu::{ConvertTryFrom as _};"] # [doc = ""] # [doc = " let jiff_offset = jiff::tz::Offset::from_seconds("] # [doc = "     5 * 60 * 60 + 30 * 60,"] # [doc = " ).unwrap();"] # [doc = " let icu_tz = icu_time::zone::UtcOffset::convert_try_from(jiff_offset)?;"] # [doc = " assert_eq!("] # [doc = "     format!(\"{icu_tz:?}\"),"] # [doc = "     \"UtcOffset(19800)\","] # [doc = " );"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [cfg (feature = "time")] impl ConvertTryFrom < JiffOffset > for IcuUtcOffset { type Error = Error ; fn convert_try_from (v : JiffOffset) -> Result < IcuUtcOffset , Error > { IcuUtcOffset :: try_from_seconds (v . seconds ()) . map_err (| err | { err ! ("failed to convert Jiff UTC offset of \
                 `{v}` to ICU4X offset: {err}" ,) }) } }
};
}
