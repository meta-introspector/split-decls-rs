// Generated macro for impl_44 (impl)
macro_rules! Depcrate_versionimpl_44 {
() => {
// Module: crate::version
// Provides: {"impl_44"}
// Dependencies: {}
impl Version { # [doc = " Creates a `Version` instance for a specific `major.minor.patch` version."] pub fn new (major : usize , minor : usize , patch : usize) -> Self { Version { major : major , minor : minor , patch : patch , } } pub fn from_command (command : & mut Command) -> Result < Self , Error > { let output = try ! (command . args (& ["--version" , "--verbose"]) . output () . map_err (error :: from_io)) ; if ! output . status . success () { return Err (error :: from_str ("could not execute rustc")) ; } let output = try ! (str :: from_utf8 (& output . stdout) . map_err (error :: from_utf8)) ; let release = match output . lines () . find (| line | line . starts_with ("release: ")) { Some (line) => & line ["release: " . len () ..] , None => return Err (error :: from_str ("could not find rustc release")) , } ; let version = match release . find ('-') { Some (i) => & release [.. i] , None => release , } ; let mut iter = version . splitn (3 , '.') ; let major = try ! (iter . next () . ok_or_else (|| error :: from_str ("missing major version"))) ; let minor = try ! (iter . next () . ok_or_else (|| error :: from_str ("missing minor version"))) ; let patch = try ! (iter . next () . ok_or_else (|| error :: from_str ("missing patch version"))) ; Ok (Version :: new (try ! (major . parse () . map_err (error :: from_num)) , try ! (minor . parse () . map_err (error :: from_num)) , try ! (patch . parse () . map_err (error :: from_num)) ,)) } }
};
}
