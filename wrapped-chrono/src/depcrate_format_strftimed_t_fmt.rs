// Generated macro for D_T_FMT (static)
macro_rules! Depcrate_format_strftimeD_T_FMT {
() => {
// Module: crate::format::strftime
// Provides: {"D_T_FMT"}
// Dependencies: {}
static D_T_FMT : & [Item < 'static >] = & [fixed (Fixed :: ShortWeekdayName) , Item :: Space (" ") , fixed (Fixed :: ShortMonthName) , Item :: Space (" ") , nums (Numeric :: Day) , Item :: Space (" ") , num0 (Numeric :: Hour) , Item :: Literal (":") , num0 (Numeric :: Minute) , Item :: Literal (":") , num0 (Numeric :: Second) , Item :: Space (" ") , num0 (Numeric :: Year) ,] ;
};
}
