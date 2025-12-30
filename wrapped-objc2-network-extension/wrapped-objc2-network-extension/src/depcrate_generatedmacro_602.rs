// Generated macro for macro_602 (macro)
macro_rules! Depcrate_generatedmacro_602 {
() => {
// Module: crate::generated
// Provides: {"macro_602"}
// Dependencies: {}
extern_class ! (# [doc = " The NETransparentProxyProvider class declares the programmatic interface for an object that implements the client side of a custom transparent network proxy solution."] # [doc = " The NETransparentProxyProvider class has the following behavior differences from its super class NEAppProxyProvider:"] # [doc = " - Returning NO from handleNewFlow: and handleNewUDPFlow:initialRemoteEndpoint: causes the flow to proceed to communicate directly with the flow's ultimate destination, instead of closing the flow with a \"Connection Refused\" error."] # [doc = " - NEDNSSettings and NEProxySettings specified within NETransparentProxyNetworkSettings are ignored. Flows that match the includedNetworkRules within NETransparentProxyNetworkSettings will use the same DNS and proxy settings that other flows on the system are currently using."] # [doc = " - Flows that are created using a \"connect by name\" API (such as Network.framework or NSURLSession) that match the includedNetworkRules will not bypass DNS resolution."] # [doc = ""] # [doc = " NETransparentProxyProvider is part of NetworkExtension.framework"] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/networkextension/netransparentproxyprovider?language=objc)"] # [unsafe (super (NEAppProxyProvider , NETunnelProvider , NEProvider , NSObject))] # [derive (Debug , PartialEq , Eq , Hash)] pub struct NETransparentProxyProvider ;) ;
};
}
