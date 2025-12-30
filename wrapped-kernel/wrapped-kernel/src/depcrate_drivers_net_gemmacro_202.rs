// Generated macro for macro_202 (macro)
macro_rules! Depcrate_drivers_net_gemmacro_202 {
() => {
// Module: crate::drivers::net::gem
// Provides: {"macro_202"}
// Dependencies: {}
register_structs ! { # [doc = " Register offsets"] Registers { (0x000 => network_control : ReadWrite < u32 , NetworkControl :: Register >) , (0x004 => network_config : ReadWrite < u32 , NetworkConfig :: Register >) , (0x008 => network_status : ReadOnly < u32 , NetworkStatus :: Register >) , (0x00C => _reserved1) , (0x010 => dma_config : ReadWrite < u32 , DMAConfig :: Register >) , (0x014 => transmit_status : ReadWrite < u32 , TransmitStatus :: Register >) , (0x018 => rx_qbar : ReadWrite < u32 >) , (0x01c => tx_qbar : ReadWrite < u32 >) , (0x020 => receive_status : ReadWrite < u32 , ReceiveStatus :: Register >) , (0x024 => int_status : ReadWrite < u32 , Interrupts :: Register >) , (0x028 => int_enable : WriteOnly < u32 , Interrupts :: Register >) , (0x02C => int_disable : WriteOnly < u32 , Interrupts :: Register >) , (0x030 => _reserved3) , (0x034 => phy_maintenance : ReadWrite < u32 , PHYMaintenance :: Register >) , (0x038 => _reserved4) , (0x088 => spec_add1_bottom : ReadWrite < u32 >) , (0x08C => spec_add1_top : ReadWrite < u32 >) , (0x090 => _reserved5) , (0x1000 => @ END) , } }
};
}
