// Generated macro for impl_61 (impl)
macro_rules! Depcrate_unstableimpl_61 {
() => {
// Module: crate::unstable
// Provides: {"impl_61"}
// Dependencies: {}
impl Features { pub fn dump_feature_usage_metrics (& self , metrics_path : PathBuf ,) -> Result < () , Box < dyn std :: error :: Error > > { # [derive (serde :: Serialize)] struct LibFeature { timestamp : u128 , symbol : String , } # [derive (serde :: Serialize)] struct LangFeature { timestamp : u128 , symbol : String , since : Option < String > , } # [derive (serde :: Serialize)] struct FeatureUsage { lib_features : Vec < LibFeature > , lang_features : Vec < LangFeature > , } let metrics_file = std :: fs :: File :: create (metrics_path) ? ; let metrics_file = std :: io :: BufWriter :: new (metrics_file) ; let now = | | { SystemTime :: now () . duration_since (UNIX_EPOCH) . expect ("system time should always be greater than the unix epoch") . as_nanos () } ; let lib_features = self . enabled_lib_features . iter () . map (| EnabledLibFeature { gate_name , .. } | LibFeature { symbol : gate_name . to_string () , timestamp : now () , }) . collect () ; let lang_features = self . enabled_lang_features . iter () . map (| EnabledLangFeature { gate_name , stable_since , .. } | LangFeature { symbol : gate_name . to_string () , since : stable_since . map (| since | since . to_string ()) , timestamp : now () , }) . collect () ; let feature_usage = FeatureUsage { lib_features , lang_features } ; serde_json :: to_writer (metrics_file , & feature_usage) ? ; Ok (()) } }
};
}
