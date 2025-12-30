// Generated macro for other_2253 (other)
macro_rules! Depcrate_generatedother_2253 {
() => {
// Module: crate::generated
// Provides: {"other_2253"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Provides MIDI messages to a clock without using CoreMIDI"] # [doc = ""] # [doc = " In some situations, a client may wish to drive a clock using MIDI Time Code or"] # [doc = " beat clock obtained from a source other than Core MIDI. To do so,"] # [doc = " construct MIDIPacketLists containing the timecode or beat clock messages,"] # [doc = " and pass them to this function."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inCAClock`: The clock object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inMIDIPacketList`: The MIDI events to be parsed."] # [doc = ""] # [doc = ""] # [doc = " Returns: An OSStatus error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_ca_clock` must be a valid pointer."] # [doc = " - `in_midi_packet_list` must be a valid pointer."] # [cfg (feature = "objc2-core-midi")] pub fn CAClockParseMIDI (in_ca_clock : CAClockRef , in_midi_packet_list : NonNull < MIDIPacketList > ,) -> OSStatus ; }
};
}
