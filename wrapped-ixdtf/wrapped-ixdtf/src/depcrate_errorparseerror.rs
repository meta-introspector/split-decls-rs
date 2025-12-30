// Generated macro for ParseError (enum)
macro_rules! Depcrate_errorParseError {
() => {
// Module: crate::error
// Provides: {"ParseError"}
// Dependencies: {}
# [non_exhaustive] # [derive (PartialEq , Clone , Copy , Debug)] # [doc = " The error returned by `ixdtf`'s parsers."] pub enum ParseError { ImplAssert , NonAsciiCodePoint , ParseFloat , AbruptEnd { location : & 'static str } , InvalidEnd , InvalidMonthRange , InvalidDayRange , DateYear , DateExtendedYear , DateMonth , DateDay , DateUnexpectedEnd , TimeRequired , TimeHour , TimeMinuteSecond , TimeSecond , FractionPart , DateSeparator , TimeSeparator , DecimalSeparator , InvalidAnnotation , AnnotationOpen , AnnotationClose , AnnotationChar , AnnotationKeyValueSeparator , AnnotationKeyLeadingChar , AnnotationKeyChar , AnnotationValueCharPostHyphen , AnnotationValueChar , InvalidMinutePrecisionOffset , CriticalDuplicateCalendar , UnrecognizedCritical , TzLeadingChar , IanaCharPostSeparator , IanaChar , UtcTimeSeparator , OffsetNeedsSign , MonthDayHyphen , DurationDisgnator , DurationValueExceededRange , DateDurationPartOrder , TimeDurationPartOrder , TimeDurationDesignator , AmbiguousTimeMonthDay , AmbiguousTimeYearMonth , InvalidMonthDay , }
};
}
