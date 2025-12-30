// Generated macro for impl_28 (impl)
macro_rules! Depcrate_civil_dateimpl_28 {
() => {
// Module: crate::civil::date
// Provides: {"impl_28"}
// Dependencies: {}
impl Ord for Date { # [inline] fn cmp (& self , other : & Date) -> core :: cmp :: Ordering { (self . year . get () , self . month . get () , self . day . get ()) . cmp (& (other . year . get () , other . month . get () , other . day . get () ,)) } }
};
}
