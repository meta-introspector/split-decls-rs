// Generated macro for unique (function)
macro_rules! Depcrateunique {
() => {
// Module: crate
// Provides: {"unique"}
// Dependencies: {}
fn unique () -> String { # [repr (C)] # [derive (Default)] pub struct Guid { pub data1 : u32 , pub data2 : u16 , pub data3 : u16 , pub data4 : [u8 ; 8] , } windows_link :: link ! ("ole32.dll" "system" fn CoCreateGuid (pguid : * mut Guid) -> i32) ; let mut guid = Guid :: default () ; unsafe { CoCreateGuid (& mut guid) } ; format ! ("{:08X?}-{:04X?}-{:04X?}-{:02X?}{:02X?}-{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}" , guid . data1 , guid . data2 , guid . data3 , guid . data4 [0] , guid . data4 [1] , guid . data4 [2] , guid . data4 [3] , guid . data4 [4] , guid . data4 [5] , guid . data4 [6] , guid . data4 [7]) }
};
}
