// Generated macro for impl_ops (module)
macro_rules! Depcrate_parseimpl_ops {
() => {
// Module: crate::parse
// Provides: {"impl_ops"}
// Dependencies: {}
mod impl_ops { use super :: Unit ; use core :: ops ; impl ops :: Add < u64 > for Unit { type Output = u64 ; fn add (self , other : u64) -> Self :: Output { self . factor () + other } } impl ops :: Add < Unit > for u64 { type Output = u64 ; fn add (self , other : Unit) -> Self :: Output { self + other . factor () } } impl ops :: Mul < u64 > for Unit { type Output = u64 ; fn mul (self , other : u64) -> Self :: Output { self . factor () * other } } impl ops :: Mul < Unit > for u64 { type Output = u64 ; fn mul (self , other : Unit) -> Self :: Output { self * other . factor () } } impl ops :: Add < f64 > for Unit { type Output = f64 ; fn add (self , other : f64) -> Self :: Output { self . factor () as f64 + other } } impl ops :: Add < Unit > for f64 { type Output = f64 ; fn add (self , other : Unit) -> Self :: Output { other . factor () as f64 + self } } impl ops :: Mul < f64 > for Unit { type Output = f64 ; fn mul (self , other : f64) -> Self :: Output { self . factor () as f64 * other } } impl ops :: Mul < Unit > for f64 { type Output = f64 ; fn mul (self , other : Unit) -> Self :: Output { other . factor () as f64 * self } } }
};
}
