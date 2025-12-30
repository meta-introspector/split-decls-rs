// Generated macro for get_events (macro)
macro_rules! Depcrate_perfcnt_intelget_events {
() => {
// Module: crate::perfcnt::intel
// Provides: {"get_events"}
// Dependencies: {}
macro_rules ! get_events { ($ format : expr) => { { let cpuid = cpuid :: CpuId :: new () ; cpuid . get_vendor_info () . map_or (None , | vf | { cpuid . get_feature_info () . map_or (None , | fi | { let vendor = vf . as_str () ; let (family , extended_model , model) = (fi . base_family_id () , fi . extended_model_id () , fi . base_model_id () ,) ; let mut writer : ModelWriter = Default :: default () ; write ! (writer , $ format , vendor , family , extended_model , model) . unwrap () ; let key = writer . as_str () ; events :: COUNTER_MAP . get (key) }) }) } } ; }
};
}
