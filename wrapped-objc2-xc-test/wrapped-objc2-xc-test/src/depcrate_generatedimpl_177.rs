// Generated macro for impl_177 (impl)
macro_rules! Depcrate_generatedimpl_177 {
() => {
// Module: crate::generated
// Provides: {"impl_177"}
// Dependencies: {}
impl XCTMeasureOptions { extern_methods ! (# [doc = " Builds a set of recommended default options for measuring."] # [doc = ""] # [doc = ""] # [doc = " Returns: An object which represents a set of default configuration options for measuring."] # [unsafe (method (defaultOptions))] # [unsafe (method_family = none)] pub fn defaultOptions () -> Retained < XCTMeasureOptions >; # [doc = " Set of options which configure how measurements are taken. The default option is XCTMeasurementInvocationNone."] # [unsafe (method (invocationOptions))] # [unsafe (method_family = none)] pub fn invocationOptions (& self) -> XCTMeasurementInvocationOptions ; # [doc = " Setter for [`invocationOptions`][Self::invocationOptions]."] # [unsafe (method (setInvocationOptions :))] # [unsafe (method_family = none)] pub fn setInvocationOptions (& self , invocation_options : XCTMeasurementInvocationOptions) ; # [doc = " The number of times the block being measured should be invoked. The default value is 5."] # [doc = " Note that the block is actually invoked `iterationCount` + 1 times, and the first iteration"] # [doc = " is discarded. This is done to reduce the chance that the first iteration will be an outlier."] # [unsafe (method (iterationCount))] # [unsafe (method_family = none)] pub fn iterationCount (& self) -> NSUInteger ; # [doc = " Setter for [`iterationCount`][Self::iterationCount]."] # [unsafe (method (setIterationCount :))] # [unsafe (method_family = none)] pub fn setIterationCount (& self , iteration_count : NSUInteger) ;) ; }
};
}
