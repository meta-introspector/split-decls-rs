// Generated macro for impl_126 (impl)
macro_rules! Depcrate_cal_copticimpl_126 {
() => {
// Module: crate::cal::coptic
// Provides: {"impl_126"}
// Dependencies: {}
impl Coptic { pub (crate) fn reference_year_from_month_day (month : types :: Month , day : u8 ,) -> Result < i32 , EcmaReferenceYearError > { let (ordinal_month , false) = (month . number () , month . is_leap ()) else { return Err (EcmaReferenceYearError :: MonthCodeNotInCalendar) ; } ; let anno_martyrum_year = if ordinal_month < 4 || (ordinal_month == 4 && day <= 22) { 1689 } else if ordinal_month == 13 && day >= 6 { 1687 } else { 1688 } ; Ok (anno_martyrum_year) } }
};
}
