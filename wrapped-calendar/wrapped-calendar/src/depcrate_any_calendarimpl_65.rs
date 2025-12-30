// Generated macro for impl_65 (impl)
macro_rules! Depcrate_any_calendarimpl_65 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_65"}
// Dependencies: {}
impl TryFrom < CalendarAlgorithm > for AnyCalendarKind { type Error = () ; fn try_from (v : CalendarAlgorithm) -> Result < Self , Self :: Error > { use CalendarAlgorithm :: * ; match v { Buddhist => Ok (AnyCalendarKind :: Buddhist) , Chinese => Ok (AnyCalendarKind :: Chinese) , Coptic => Ok (AnyCalendarKind :: Coptic) , Dangi => Ok (AnyCalendarKind :: Dangi) , Ethioaa => Ok (AnyCalendarKind :: EthiopianAmeteAlem) , Ethiopic => Ok (AnyCalendarKind :: Ethiopian) , Gregory => Ok (AnyCalendarKind :: Gregorian) , Hebrew => Ok (AnyCalendarKind :: Hebrew) , Indian => Ok (AnyCalendarKind :: Indian) , Hijri (None) => Err (()) , Hijri (Some (HijriCalendarAlgorithm :: Umalqura)) => Ok (AnyCalendarKind :: HijriUmmAlQura) , Hijri (Some (HijriCalendarAlgorithm :: Tbla)) => { Ok (AnyCalendarKind :: HijriTabularTypeIIThursday) } Hijri (Some (HijriCalendarAlgorithm :: Civil)) => { Ok (AnyCalendarKind :: HijriTabularTypeIIFriday) } Hijri (Some (HijriCalendarAlgorithm :: Rgsa)) => Ok (AnyCalendarKind :: HijriSimulatedMecca) , Iso8601 => Ok (AnyCalendarKind :: Iso) , Japanese => Ok (AnyCalendarKind :: Japanese) , Persian => Ok (AnyCalendarKind :: Persian) , Roc => Ok (AnyCalendarKind :: Roc) , _ => { debug_assert ! (false , "unknown calendar algorithm {v:?}") ; Err (()) } } } }
};
}
