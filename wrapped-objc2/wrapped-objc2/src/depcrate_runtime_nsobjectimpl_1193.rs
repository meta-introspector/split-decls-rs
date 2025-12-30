// Generated macro for impl_1193 (impl)
macro_rules! Depcrate_runtime_nsobjectimpl_1193 {
() => {
// Module: crate::runtime::nsobject
// Provides: {"impl_1193"}
// Dependencies: {}
# [allow (non_snake_case)] impl NSObject { extern_methods ! (# [doc = " Create a new empty `NSObject`."] # [doc = ""] # [doc = " This method is a shorthand for calling [`alloc`][AnyThread::alloc]"] # [doc = " and then [`init`][Self::init]."] # [unsafe (method (new))] # [unsafe (method_family = new)] pub fn new () -> Retained < Self >; # [doc = " Initialize an already allocated object."] # [doc = ""] # [doc = " See [Apple's documentation][apple-doc] for details."] # [doc = ""] # [doc = " [apple-doc]: https://developer.apple.com/documentation/objectivec/nsobject/1418641-init?language=objc"] # [doc = ""] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use objc2::runtime::NSObject;"] # [doc = " use objc2::AnyThread;"] # [doc = ""] # [doc = " let obj = NSObject::init(NSObject::alloc());"] # [doc = " ```"] # [unsafe (method (init))] # [unsafe (method_family = init)] pub fn init (this : Allocated < Self >) -> Retained < Self >; # [unsafe (method (doesNotRecognizeSelector :))] # [unsafe (method_family = none)] fn doesNotRecognizeSelector_inner (& self , sel : Sel) ;) ; # [doc = " Handle messages the object doesn’t recognize."] # [doc = ""] # [doc = " See [Apple's documentation][apple-doc] for details."] # [doc = ""] # [doc = " [apple-doc]: https://developer.apple.com/documentation/objectivec/nsobject/1418637-doesnotrecognizeselector?language=objc"] pub fn doesNotRecognizeSelector (& self , sel : Sel) -> ! { self . doesNotRecognizeSelector_inner (sel) ; unreachable ! ("doesNotRecognizeSelector: should not return") } }
};
}
