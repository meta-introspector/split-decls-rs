// Generated macro for phy_read (function)
macro_rules! Depcrate_drivers_net_gemphy_read {
() => {
// Module: crate::drivers::net::gem
// Provides: {"phy_read"}
// Dependencies: {}
unsafe fn phy_read (gem : * mut Registers , addr : u32 , reg : PhyReg) -> u16 { unsafe { wait_for_mdio (gem) ; (* gem) . phy_maintenance . write (PHYMaintenance :: CLAUSE_22 :: SET + PHYMaintenance :: OP :: READ + PHYMaintenance :: ADDR . val (addr) + PHYMaintenance :: REG . val (reg as u32) + PHYMaintenance :: MUST_10 :: MUST_BE_10 ,) ; wait_for_mdio (gem) ; (* gem) . phy_maintenance . read (PHYMaintenance :: DATA) as u16 } }
};
}
