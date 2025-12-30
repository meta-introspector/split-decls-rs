// Generated macro for null_as_f64_nan (function)
macro_rules! Depcrate_metricsnull_as_f64_nan {
() => {
// Module: crate::metrics
// Provides: {"null_as_f64_nan"}
// Dependencies: {}
fn null_as_f64_nan < 'de , D : serde :: Deserializer < 'de > > (d : D) -> Result < f64 , D :: Error > { use serde :: Deserialize as _ ; Option :: < f64 > :: deserialize (d) . map (| f | f . unwrap_or (f64 :: NAN)) }
};
}
