// Generated macro for HEBREW_LUNATION_TIME (const)
macro_rules! Depcrate_hebrew_keviyahHEBREW_LUNATION_TIME {
() => {
// Module: crate::hebrew_keviyah
// Provides: {"HEBREW_LUNATION_TIME"}
// Dependencies: {}
# [doc = " The amount of time a Hebrew lunation takes (in ḥalakim). This is not exactly the amount of time"] # [doc = " taken by one revolution of the moon (the real world seldom has events that are perfect integer"] # [doc = " multiples of 1080ths of an hour), but it is what the Hebrew calendar uses. This does mean that"] # [doc = " there will be drift over time with the actual state of the celestial sphere, however that is"] # [doc = " irrelevant since the actual state of the celestial sphere is not what is used for the Hebrew"] # [doc = " calendar."] # [doc = ""] # [doc = " This is 29-12-793 in zero-indexed notation. It is equal to 765433ḥal."] # [doc = " From Adjler Appendix A"] const HEBREW_LUNATION_TIME : i32 = ḥal ! (0 - indexed 29 - 12 - 793) ;
};
}
