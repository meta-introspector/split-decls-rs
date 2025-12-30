// Generated macro for interact_buzy_loop (function)
macro_rules! Depcrate_interact_sessioninteract_buzy_loop {
() => {
// Module: crate::interact::session
// Provides: {"interact_buzy_loop"}
// Dependencies: {}
# [cfg (all (windows , not (feature = "async") , not (feature = "polling")))] fn interact_buzy_loop < S , O , I , C > (s : & mut InteractSession < S , I , O , C >) -> ExpectResult < bool > where S : Healthcheck + NonBlocking + Write + Read , O : Write , I : Read , { let mut buf = [0 ; 512] ; loop { if ! s . session . is_alive () ? { return Ok (false) ; } if let Some (n) = try_read (& mut s . session , & mut buf) ? { let eof = n == 0 ; let buf = & buf [.. n] ; let buf = call_filter (s . opts . output_filter . as_mut () , buf) ? ; let exit = run_action_output (s , & buf , eof) ? ; if eof || exit { return Ok (true) ; } spin_write (& mut s . output , & buf) ? ; spin_flush (& mut s . output) ? ; } match s . input . read (& mut buf) { Ok (n) => { let eof = n == 0 ; let buf = & buf [.. n] ; let buf = call_filter (s . opts . input_filter . as_mut () , buf) ? ; let exit = run_action_input (s , & buf , eof) ? ; if eof | exit { return Ok (true) ; } let escape_char_position = buf . iter () . position (| c | * c == s . escape_character) ; match escape_char_position { Some (pos) => { s . session . write_all (& buf [.. pos]) ? ; return Ok (true) ; } None => { s . session . write_all (& buf [..]) ? ; } } } Err (err) if err . kind () == ErrorKind :: WouldBlock => { } Err (err) => return Err (err . into ()) , } let exit = run_action_idle (s , & [] , false) ? ; if exit { return Ok (true) ; } } }
};
}
