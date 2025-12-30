// Generated macro for impl_78 (impl)
macro_rules! Depcrate_discoverimpl_78 {
() => {
// Module: crate::discover
// Provides: {"impl_78"}
// Dependencies: {}
impl DiscoverCommand { # [doc = " Create a new [DiscoverCommand]."] pub (crate) fn new (sender : Sender < DiscoverProjectMessage > , command : Vec < String >) -> Self { Self { sender , command } } # [doc = " Spawn the command inside [Discover] and report progress, if any."] pub (crate) fn spawn (& self , discover_arg : DiscoverArgument , current_dir : & Path ,) -> io :: Result < DiscoverHandle > { let command = & self . command [0] ; let args = & self . command [1 ..] ; let args : Vec < String > = args . iter () . map (| arg | { if arg == ARG_PLACEHOLDER { serde_json :: to_string (& discover_arg) . expect ("Unable to serialize args") } else { arg . to_owned () } }) . collect () ; let mut cmd = toolchain :: command (command , current_dir , & FxHashMap :: default ()) ; cmd . args (args) ; Ok (DiscoverHandle { _handle : CommandHandle :: spawn (cmd , DiscoverProjectParser , self . sender . clone ()) ? , span : info_span ! ("discover_command") . entered () , }) } }
};
}
