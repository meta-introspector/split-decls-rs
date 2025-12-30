// Generated macro for Era (enum)
macro_rules! Depcrate_civilEra {
() => {
// Module: crate::civil
// Provides: {"Era"}
// Dependencies: {}
# [doc = " The era corresponding to a particular year."] # [doc = ""] # [doc = " The BCE era corresponds to years less than or equal to `0`, while the CE"] # [doc = " era corresponds to years greater than `0`."] # [doc = ""] # [doc = " In particular, this crate allows years to be negative and also to be `0`,"] # [doc = " which is contrary to the common practice of excluding the year `0` when"] # [doc = " writing dates for the Gregorian calendar. Moreover, common practice eschews"] # [doc = " negative years in favor of labeling a year with an era notation. That is,"] # [doc = " the year `1 BCE` is year `0` in this crate. The year `2 BCE` is the year"] # [doc = " `-1` in this crate."] # [doc = ""] # [doc = " To get the year in its era format, use [`Date::era_year`]."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub enum Era { # [doc = " The \"before common era\" era."] # [doc = ""] # [doc = " This corresponds to all years less than or equal to `0`."] # [doc = ""] # [doc = " This is precisely equivalent to the \"BC\" or \"before Christ\" era."] BCE , # [doc = " The \"common era\" era."] # [doc = ""] # [doc = " This corresponds to all years greater than `0`."] # [doc = ""] # [doc = " This is precisely equivalent to the \"AD\" or \"anno Domini\" or \"in the"] # [doc = " year of the Lord\" era."] CE , }
};
}
