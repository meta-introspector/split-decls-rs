// Generated macro for impl_60 (impl)
macro_rules! Depcrate_pwmimpl_60 {
() => {
// Module: crate::pwm
// Provides: {"impl_60"}
// Dependencies: {}
impl < T : SetDutyCycle + ? Sized > SetDutyCycle for & mut T { # [inline] fn max_duty_cycle (& self) -> u16 { T :: max_duty_cycle (self) } # [inline] fn set_duty_cycle (& mut self , duty : u16) -> Result < () , Self :: Error > { T :: set_duty_cycle (self , duty) } # [inline] fn set_duty_cycle_fully_off (& mut self) -> Result < () , Self :: Error > { T :: set_duty_cycle_fully_off (self) } # [inline] fn set_duty_cycle_fully_on (& mut self) -> Result < () , Self :: Error > { T :: set_duty_cycle_fully_on (self) } # [inline] fn set_duty_cycle_fraction (& mut self , num : u16 , denom : u16) -> Result < () , Self :: Error > { T :: set_duty_cycle_fraction (self , num , denom) } # [inline] fn set_duty_cycle_percent (& mut self , percent : u8) -> Result < () , Self :: Error > { T :: set_duty_cycle_percent (self , percent) } }
};
}
