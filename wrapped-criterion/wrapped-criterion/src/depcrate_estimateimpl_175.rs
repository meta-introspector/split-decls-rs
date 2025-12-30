// Generated macro for impl_175 (impl)
macro_rules! Depcrate_estimateimpl_175 {
() => {
// Module: crate::estimate
// Provides: {"impl_175"}
// Dependencies: {}
impl Distributions { pub fn typical (& self) -> & Distribution < f64 > { self . slope . as_ref () . unwrap_or (& self . mean) } pub fn get (& self , stat : Statistic) -> Option < & Distribution < f64 > > { match stat { Statistic :: Mean => Some (& self . mean) , Statistic :: Median => Some (& self . median) , Statistic :: MedianAbsDev => Some (& self . median_abs_dev) , Statistic :: Slope => self . slope . as_ref () , Statistic :: StdDev => Some (& self . std_dev) , Statistic :: Typical => Some (self . typical ()) , } } }
};
}
