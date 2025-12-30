// Generated macro for match_cal (macro)
macro_rules! Depcrate_any_calendarmatch_cal {
() => {
// Module: crate::any_calendar
// Provides: {"match_cal"}
// Dependencies: {}
macro_rules ! match_cal { (match $ cal : ident : ($ cal_matched : ident) => $ e : expr) => { match $ cal { & Self :: Buddhist (ref $ cal_matched) => AnyDateInner :: Buddhist ($ e) , & Self :: Chinese (ref $ cal_matched) => AnyDateInner :: Chinese ($ e) , & Self :: Coptic (ref $ cal_matched) => AnyDateInner :: Coptic ($ e) , & Self :: Dangi (ref $ cal_matched) => AnyDateInner :: Dangi ($ e) , & Self :: Ethiopian (ref $ cal_matched) => AnyDateInner :: Ethiopian ($ e) , & Self :: Gregorian (ref $ cal_matched) => AnyDateInner :: Gregorian ($ e) , & Self :: Hebrew (ref $ cal_matched) => AnyDateInner :: Hebrew ($ e) , & Self :: HijriSimulated (ref $ cal_matched) => AnyDateInner :: HijriSimulated ($ e) , & Self :: HijriTabular (ref $ cal_matched) => AnyDateInner :: HijriTabular ($ e , $ cal_matched . 0) , & Self :: HijriUmmAlQura (ref $ cal_matched) => AnyDateInner :: HijriUmmAlQura ($ e) , & Self :: Indian (ref $ cal_matched) => AnyDateInner :: Indian ($ e) , & Self :: Iso (ref $ cal_matched) => AnyDateInner :: Iso ($ e) , & Self :: Japanese (ref $ cal_matched) => AnyDateInner :: Japanese ($ e) , & Self :: JapaneseExtended (ref $ cal_matched) => AnyDateInner :: JapaneseExtended ($ e) , & Self :: Persian (ref $ cal_matched) => AnyDateInner :: Persian ($ e) , & Self :: Roc (ref $ cal_matched) => AnyDateInner :: Roc ($ e) , } } ; }
};
}
