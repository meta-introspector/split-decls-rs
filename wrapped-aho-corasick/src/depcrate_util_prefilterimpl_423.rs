// Generated macro for impl_423 (impl)
macro_rules! Depcrate_util_prefilterimpl_423 {
() => {
// Module: crate::util::prefilter
// Provides: {"impl_423"}
// Dependencies: {}
impl MemmemBuilder { fn build (& self) -> Option < Prefilter > { # [cfg (all (feature = "std" , feature = "perf-literal"))] fn imp (builder : & MemmemBuilder) -> Option < Prefilter > { let pattern = builder . one . as_ref () ? ; assert_eq ! (1 , builder . count) ; let finder = Arc :: new (Memmem (memchr :: memmem :: Finder :: new (pattern) . into_owned () ,)) ; let memory_usage = pattern . len () ; Some (Prefilter { finder , memory_usage }) } # [cfg (not (all (feature = "std" , feature = "perf-literal")))] fn imp (_ : & MemmemBuilder) -> Option < Prefilter > { None } imp (self) } fn add (& mut self , bytes : & [u8]) { self . count += 1 ; if self . count == 1 { self . one = Some (bytes . to_vec ()) ; } else { self . one = None ; } } }
};
}
