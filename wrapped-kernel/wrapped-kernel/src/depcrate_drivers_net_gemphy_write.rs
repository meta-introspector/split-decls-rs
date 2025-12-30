// Generated macro for phy_write (function)
macro_rules! Depcrate_drivers_net_gemphy_write {
() => {
// Module: crate::drivers::net::gem
// Provides: {"phy_write"}
// Dependencies: {}
unsafe fn phy_write (gem : * mut Registers , addr : u32 , reg : PhyReg , data : u16) { unsafe { wait_for_mdio (gem) ; (* gem) . phy_maintenance . write (PHYMaintenance :: CLAUSE_22 :: SET + PHYMaintenance :: OP :: WRITE + PHYMaintenance :: ADDR . val (addr) + PHYMaintenance :: REG . val (reg as u32) + PHYMaintenance :: MUST_10 :: MUST_BE_10 + PHYMaintenance :: DATA . val (data . into ()) ,) ; wait_for_mdio (gem) ; } }
};
}
