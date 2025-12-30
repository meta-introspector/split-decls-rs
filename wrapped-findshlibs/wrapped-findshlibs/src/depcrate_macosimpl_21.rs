// Generated macro for impl_21 (impl)
macro_rules! Depcrate_macosimpl_21 {
() => {
// Module: crate::macos
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a > Iterator for SegmentIter < 'a > { type Item = Segment < 'a > ; fn next (& mut self) -> Option < Self :: Item > { while self . num_commands > 0 { self . num_commands -= 1 ; let this_command = unsafe { self . commands . as_ref () . unwrap () } ; let command_size = this_command . cmdsize as isize ; match this_command . cmd { libc :: LC_SEGMENT => { let segment = self . commands as * const libc :: segment_command ; let segment = unsafe { segment . as_ref () . unwrap () } ; self . commands = unsafe { (self . commands as * const u8) . offset (command_size) as * const _ } ; return Some (Segment :: Segment32 (segment)) ; } libc :: LC_SEGMENT_64 => { let segment = self . commands as * const libc :: segment_command_64 ; let segment = unsafe { segment . as_ref () . unwrap () } ; self . commands = unsafe { (self . commands as * const u8) . offset (command_size) as * const _ } ; return Some (Segment :: Segment64 (segment)) ; } _ => { self . commands = unsafe { (self . commands as * const u8) . offset (command_size) as * const _ } ; continue ; } } } None } }
};
}
