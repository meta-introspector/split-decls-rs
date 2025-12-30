// Generated macro for impl_691 (impl)
macro_rules! Depcrate_dynamic_subscriptionimpl_691 {
() => {
// Module: crate::dynamic::subscription
// Provides: {"impl_691"}
// Dependencies: {}
impl Debug for SubscriptionField { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Field") . field ("name" , & self . name) . field ("description" , & self . description) . field ("arguments" , & self . arguments) . field ("ty" , & self . ty) . field ("deprecation" , & self . deprecation) . finish () } }
};
}
