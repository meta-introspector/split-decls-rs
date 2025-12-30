// Generated macro for impl_130 (impl)
macro_rules! Depcrate_hebrew_keviyahimpl_130 {
() => {
// Module: crate::hebrew_keviyah
// Provides: {"impl_130"}
// Dependencies: {}
impl YearInfo { # [doc = " Compute the YearInfo for a given year"] # [inline] pub fn compute_for (h_year : i32) -> Self { let (mut weeks_since_beharad , ḥalakim) = molad_details (h_year) ; let cycle_type = MetonicCycleType :: for_h_year (h_year) ; let keviyah = keviyah_for (cycle_type , ḥalakim) ; if ḥalakim >= ḥal ! (7 - 18 - 0) { weeks_since_beharad += 1 ; } Self { keviyah , weeks_since_beharad , } } # [doc = " Returns the YearInfo and h_year for the year containing `date`"] # [doc = ""] # [doc = " This will clamp the R.D. such that the hebrew year is within range for i32"] # [inline] pub fn year_containing_rd (date : RataDie) -> (Self , i32) { let mut h_year = i64_to_saturated_i32 ((date - HEBREW_EPOCH) * 98496 / 35975351 + (date >= HEBREW_EPOCH) as i64 ,) ; let mut year = Self :: compute_for (h_year) ; if date < year . new_year () && h_year > i32 :: MIN { h_year -= 1 ; year = Self :: compute_for (h_year) } else if date >= year . new_year () + year . keviyah . year_length () as i64 && h_year < i32 :: MAX { h_year += 1 ; year = Self :: compute_for (h_year) } (year , h_year) } # [doc = " Compute the date of New Year's Day"] # [inline] pub fn new_year (self) -> RataDie { const BEHARAD_START_OF_YEAR : StartOfYear = StartOfYear :: Monday ; let days_since_beharad = (self . weeks_since_beharad * 7) + self . keviyah . start_of_year () as i64 - BEHARAD_START_OF_YEAR as i64 ; HEBREW_EPOCH + days_since_beharad } }
};
}
