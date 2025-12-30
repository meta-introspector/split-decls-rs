// Generated macro for NEHotspotHelperHandler (type)
macro_rules! Depcrate_generatedNEHotspotHelperHandler {
() => {
// Module: crate::generated
// Provides: {"NEHotspotHelperHandler"}
// Dependencies: {}
# [doc = " The type definition for the HotspotHelper's command handler block."] # [doc = ""] # [doc = " The application provides a block of this type when it"] # [doc = " invokes the +[NEHotspotHelper registerWithOptions:queue:handler] method."] # [doc = " The block is invoked every time there is a command to be processed."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/networkextension/nehotspothelperhandler?language=objc)"] # [cfg (feature = "block2")] pub type NEHotspotHelperHandler = * mut block2 :: DynBlock < dyn Fn (NonNull < NEHotspotHelperCommand >) > ;
};
}
