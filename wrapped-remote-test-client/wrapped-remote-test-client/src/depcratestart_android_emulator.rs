// Generated macro for start_android_emulator (function)
macro_rules! Depcratestart_android_emulator {
() => {
// Module: crate
// Provides: {"start_android_emulator"}
// Dependencies: {}
fn start_android_emulator (server : & Path) { println ! ("waiting for device to come online") ; let status = Command :: new ("adb") . arg ("wait-for-device") . status () . unwrap () ; assert ! (status . success ()) ; println ! ("pushing server") ; let status = Command :: new ("adb") . arg ("push") . arg (server) . arg ("/data/local/tmp/testd") . status () . unwrap () ; assert ! (status . success ()) ; println ! ("forwarding tcp") ; let status = Command :: new ("adb") . arg ("forward") . arg ("tcp:12345") . arg ("tcp:12345") . status () . unwrap () ; assert ! (status . success ()) ; println ! ("executing server") ; Command :: new ("adb") . arg ("shell") . arg ("/data/local/tmp/testd") . spawn () . unwrap () ; }
};
}
