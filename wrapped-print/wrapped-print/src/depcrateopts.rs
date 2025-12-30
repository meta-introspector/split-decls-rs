// Generated macro for Opts (struct)
macro_rules! DepcrateOpts {
() => {
// Module: crate
// Provides: {"Opts"}
// Dependencies: {}
# [doc = " Prints defmt-encoded logs to stdout"] # [derive (Parser , Clone)] # [command (name = "defmt-print")] struct Opts { # [doc = " The firmware running on the device being logged"] # [arg (short , required = true , conflicts_with ("version"))] elf : Option < PathBuf > , # [doc = " Emit logs in JSON format"] # [arg (long)] json : bool , # [doc = " A format string for target-generated logs"] # [arg (long)] log_format : Option < String > , # [doc = " A format string for host-generated logs"] # [arg (long)] host_log_format : Option < String > , # [doc = " Log any malformed defmt frames that are being skipped"] # [arg (long)] show_skipped_frames : bool , # [doc = " Print extra detail"] # [arg (short , long)] verbose : bool , # [doc = " Print the version number, and quit"] # [arg (short = 'V' , long)] version : bool , # [doc = " Reload the ELF file when it changes"] # [arg (short , long)] watch_elf : bool , # [doc = " Which operation to perform"] # [command (subcommand)] command : Option < Command > , }
};
}
