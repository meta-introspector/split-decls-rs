macro_rules! deps {
    () => {
        Operation!();
        Process!();
        Driver!();
        State!();
        Client!();
        Key!();
        Error!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl State { # [doc = " Obtain a process as defined in `driver` suitable for a given `operation. `rela_path` may be used to substitute the current"] # [doc = " file for use in the invoked `SingleFile` process."] # [doc = ""] # [doc = " Note that if a long-running process is defined, the `operation` isn't relevant and capabilities are to be checked by the caller."] pub fn maybe_launch_process (& mut self , driver : & Driver , operation : Operation , rela_path : & BStr ,) -> Result < Option < Process < '_ > > , Error > { match driver . process . as_ref () { Some (process) => { let client = match self . running . remove (process) { Some (c) => c , None => { let (child , cmd) = spawn_driver (process . clone () , & self . context) ? ; process :: Client :: handshake (child , "git-filter" , & [2] , & ["clean" , "smudge" , "delay"]) . map_err (| err | Error :: ProcessHandshake { source : err , command : cmd , } ,) ? } } ; self . running . insert (process . clone () , client) ; let client = self . running . get_mut (process) . expect ("just inserted") ; Ok (Some (Process :: MultiFile { client , key : driver :: Key (process . to_owned ()) , })) } None => { let cmd = match operation { Operation :: Clean => driver . clean . as_ref () . map (| cmd | substitute_f_parameter (cmd . as_ref () , rela_path)) , Operation :: Smudge => driver . smudge . as_ref () . map (| cmd | substitute_f_parameter (cmd . as_ref () , rela_path)) , } ; let cmd = match cmd { Some (cmd) => cmd , None => return Ok (None) , } ; let (child , command) = spawn_driver (cmd , & self . context) ? ; Ok (Some (Process :: SingleFile { child , command })) } } } }
    };
}

impl_38!();