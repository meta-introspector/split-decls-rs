// Generated macro for impl_697 (impl)
macro_rules! Depcrate_usage_lifetimesimpl_697 {
() => {
// Module: crate::usage::lifetimes
// Provides: {"impl_697"}
// Dependencies: {}
impl < 'i , I , T > CollectLifetimes for T where T : IntoIterator < Item = & 'i I > , I : 'i + UsesLifetimes , { fn collect_lifetimes < 'a > (self , options : & Options , lifetimes : & 'a LifetimeSet ,) -> LifetimeRefSet < 'a > { self . into_iter () . fold (Default :: default () , | mut state , value | { state . extend (value . uses_lifetimes (options , lifetimes)) ; state }) } fn collect_lifetimes_cloned (self , options : & Options , lifetimes : & LifetimeSet) -> LifetimeSet { self . collect_lifetimes (options , lifetimes) . into_iter () . cloned () . collect () } }
};
}
