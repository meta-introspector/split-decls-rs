// Generated macro for notmain (function)
macro_rules! Depcratenotmain {
() => {
// Module: crate
// Provides: {"notmain"}
// Dependencies: {}
fn notmain () -> Result < Option < i32 > , anyhow :: Error > { let args = env :: args () . skip (1) . collect :: < Vec < _ > > () ; if args . len () != 1 { bail ! ("expected exactly one argument. Syntax: `qemu-run <path-to-elf>`") ; } let path = & args [0] ; let bytes = fs :: read (path) ? ; let table = if env :: var_os ("QEMU_RUN_IGNORE_VERSION") . is_some () { Table :: parse_ignore_version (& bytes) } else { Table :: parse (& bytes) } ; let table = table ? . ok_or_else (| | anyhow ! ("`.defmt` section not found")) ? ; let mut child = KillOnDrop (Command :: new ("qemu-system-arm") . args (["-cpu" , "cortex-m3" , "-machine" , "lm3s6965evb" , "-nographic" , "-monitor" , "none" , "-semihosting-config" , "enable=on,target=native" , "-kernel" ,]) . arg (path) . stdout (Stdio :: piped ()) . spawn () . expect ("Error running qemu-system-arm; perhaps you haven't installed it yet?") ,) ; let mut stdout = child . 0 . stdout . take () . ok_or_else (| | anyhow ! ("failed to acquire child's stdout handle")) ? ; let mut decoder = table . new_stream_decoder () ; let mut readbuf = [0 ; 256] ; let exit_code ; loop { let n = stdout . read (& mut readbuf) ? ; decoder . received (& readbuf [.. n]) ; decode (& mut * decoder) ? ; if let Some (status) = child . 0 . try_wait () ? { exit_code = status . code () ; let mut data = Vec :: new () ; stdout . read_to_end (& mut data) ? ; decoder . received (& data) ; decode (& mut * decoder) ? ; break ; } } Ok (exit_code) }
};
}
