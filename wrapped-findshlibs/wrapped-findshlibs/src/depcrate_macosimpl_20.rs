// Generated macro for impl_20 (impl)
macro_rules! Depcrate_macosimpl_20 {
() => {
// Module: crate::macos
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'a > SegmentIter < 'a > { fn find_uuid (& self) -> Option < [u8 ; 16] > { let mut num_commands = self . num_commands ; let mut commands = self . commands ; while num_commands > 0 { num_commands -= 1 ; let this_command = unsafe { commands . as_ref () . unwrap () } ; let command_size = this_command . cmdsize as isize ; if let LC_UUID = this_command . cmd { let uuid_cmd = commands as * const uuid_command ; return Some (unsafe { (* uuid_cmd) . uuid }) ; } commands = unsafe { (commands as * const u8) . offset (command_size) as * const _ } ; } None } }
};
}
