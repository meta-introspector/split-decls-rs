// Generated macro for impl_310 (impl)
macro_rules! Depcrate_generatedimpl_310 {
() => {
// Module: crate::generated
// Provides: {"impl_310"}
// Dependencies: {}
impl NEFilterPacketProvider { extern_methods ! (# [doc = " This function is used to delay a packet currently presented by packetHandler."] # [doc = " This function is only valid within the packetHandler block and a verdict of"] # [doc = " NEFilterPacketProviderVerdictDelay must be returned after a packet is delayed.  A delayed"] # [doc = " packet will be prevented from continuing its journey through the networking stack until"] # [doc = " it is either allowed by calling allow() or is dropped by being released."] # [doc = ""] # [doc = " Parameter `context`: The context of the current packet filter which is passed to the packetHandler block."] # [doc = " The packetHandler block must pass this context when calling delayCurrentPacket()."] # [unsafe (method (delayCurrentPacket :))] # [unsafe (method_family = none)] pub unsafe fn delayCurrentPacket (& self , context : & NEFilterPacketContext ,) -> Retained < NEPacket >; # [doc = " This function is used to allow a previously-delayed packet to continue its journey into or out of the networking stack."] # [doc = ""] # [doc = " Parameter `packet`: A NEPacket object that contains the data of the packet that was previously delayed by the NEFilterPacketProvider."] # [unsafe (method (allowPacket :))] # [unsafe (method_family = none)] pub unsafe fn allowPacket (& self , packet : & NEPacket) ;) ; }
};
}
