// Generated macro for NAME (static)
macro_rules! DepcrateNAME {
() => {
// Module: crate
// Provides: {"NAME"}
// Dependencies: {}
# [doc = " The name of our channel."] # [doc = ""] # [doc = " This is in a data section, so the whole RTT header can be read from RAM."] # [doc = " This is useful if flash access gets disabled by the firmware at runtime."] # [cfg_attr (target_os = "macos" , link_section = ".data,defmt-rtt.NAME")] # [cfg_attr (not (target_os = "macos") , link_section = ".data.defmt-rtt.NAME")] static NAME : [u8 ; 6] = * b"defmt\0" ;
};
}
