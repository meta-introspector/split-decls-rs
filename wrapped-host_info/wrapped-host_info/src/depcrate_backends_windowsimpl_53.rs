// Generated macro for impl_53 (impl)
macro_rules! Depcrate_backends_windowsimpl_53 {
() => {
// Module: crate::backends::windows
// Provides: {"impl_53"}
// Dependencies: {}
impl HostInfoBackend for WindowsHostInfoBackend { fn requested_locales () -> Result < Vec < Locale > , HostInfoError > { Ok (Self :: raw_requested_locales () ? . into_iter () . filter_map (| s | { WindowsLocale :: try_from_str (& s) . map_err (| _ | HostInfoError :: HostLocaleError) . and_then (| wl | Locale :: try_from (wl) . map_err (Into :: into)) . ok () }) . collect ()) } fn calendar () -> Result < Option < CalendarAlgorithm > , HostInfoError > { Ok (Self :: raw_calendar () ? . and_then (| raw | { let canonical = match raw . as_str () { "GregorianCalendar" => "gregory" , "JapaneseCalendar" => "japanese" , "TaiwanCalendar" => "roc" , "KoreanCalendar" => "dangi" , "HebrewCalendar" => "hebrew" , "HijriCalendar" => "islamic" , "UmmAlQuraCalendar" => "islamic-umalqura" , "PersianCalendar" => "persian" , "ThaiCalendar" => "buddhist" , "JulianCalendar" => "julian" , r => r , } ; unicode :: Value :: from_str (canonical) . ok () }) . and_then (| value | CalendarAlgorithm :: try_from (& value) . ok ())) } }
};
}
