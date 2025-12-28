macro_rules! ask {
    () => {
        # [doc = " Ask the user given a `prompt`, returning the result."] pub fn ask (prompt : & str , opts : & Options < '_ >) -> Result < String , Error > { if let Some (askpass) = opts . askpass . as_deref () { match gix_command :: prepare (askpass) . arg (prompt) . spawn () { Ok (cmd) => { if let Some (mut stdout) = cmd . wait_with_output () . ok () . and_then (| out | String :: from_utf8 (out . stdout) . ok ()) { if stdout . ends_with ('\n') { stdout . pop () ; } if stdout . ends_with ('\r') { stdout . pop () ; } return Ok (stdout) ; } } Err (err) => eprintln ! ("Cannot run askpass program: '{askpass}' with error: {err}" , askpass = askpass . display ()) , } } imp :: ask (prompt , opts) }
    };
}

ask!()