// Generated macro for wait_for_mdio (function)
macro_rules! Depcrate_drivers_net_gemwait_for_mdio {
() => {
// Module: crate::drivers::net::gem
// Provides: {"wait_for_mdio"}
// Dependencies: {}
unsafe fn wait_for_mdio (gem : * mut Registers) { unsafe { while ! (* gem) . network_status . is_set (NetworkStatus :: PHY_MGMT_IDLE) { } } }
};
}
