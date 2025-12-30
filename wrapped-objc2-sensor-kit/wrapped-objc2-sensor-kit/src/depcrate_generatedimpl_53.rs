// Generated macro for impl_53 (impl)
macro_rules! Depcrate_generatedimpl_53 {
() => {
// Module: crate::generated
// Provides: {"impl_53"}
// Dependencies: {}
impl < SampleType : Message > SRFetchResult < SampleType > { extern_methods ! (# [doc = " Retrieves the resultant sample"] # [doc = ""] # [doc = " The caller is expected to know what the result type should be"] # [doc = ""] # [doc = " Note: This may thrown an exception if the sample could not be constructed from"] # [doc = " the data in the datastore"] # [unsafe (method (sample))] # [unsafe (method_family = none)] pub unsafe fn sample (& self) -> Retained < SampleType >; # [cfg (feature = "objc2-core-foundation")] # [doc = " the timestamp the sample was written to the data store"] # [unsafe (method (timestamp))] # [unsafe (method_family = none)] pub unsafe fn timestamp (& self) -> SRAbsoluteTime ; # [unsafe (method (init))] # [unsafe (method_family = init)] pub unsafe fn init (this : Allocated < Self >) -> Retained < Self >; # [unsafe (method (new))] # [unsafe (method_family = new)] pub unsafe fn new () -> Retained < Self >;) ; }
};
}
