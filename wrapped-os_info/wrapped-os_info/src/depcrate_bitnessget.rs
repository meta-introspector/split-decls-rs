// Generated macro for get (function)
macro_rules! Depcrate_bitnessget {
() => {
// Module: crate::bitness
// Provides: {"get"}
// Dependencies: {}
# [cfg (target_os = "aix")] pub fn get () -> Bitness { match & Command :: new ("prtconf") . arg ("-c") . output () { Ok (Output { stdout , .. }) if stdout == b"CPU Type: 64-bit\n" => Bitness :: X64 , Ok (Output { stdout , .. }) if stdout == b"CPU Type: 32-bit\n" => Bitness :: X32 , _ => Bitness :: Unknown , } }
};
}
