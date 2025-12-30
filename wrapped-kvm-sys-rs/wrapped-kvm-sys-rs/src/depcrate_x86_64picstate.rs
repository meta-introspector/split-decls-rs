// Generated macro for PicState (struct)
macro_rules! Depcrate_x86_64PicState {
() => {
// Module: crate::x86_64
// Provides: {"PicState"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Debug)] pub struct PicState { pub last_irr : u8 , pub irr : u8 , pub imr : u8 , pub isr : u8 , pub priority_add : u8 , pub irq_base : u8 , pub read_reg_select : u8 , pub poll : u8 , pub special_mask : u8 , pub init_state : u8 , pub auto_eoi : u8 , pub rotate_on_auto_eoi : u8 , pub special_fully_nested_mode : u8 , pub init4 : u8 , pub elcr : u8 , pub elcr_mask : u8 , }
};
}
