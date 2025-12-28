macro_rules! deps {
    () => {
        OpenError!();
    };
}

macro_rules! wait_child {
    () => {
        deps!();
        # [cfg (not (target_os = "windows"))] fn wait_child (child : & mut std :: process :: Child , cmd_name : & 'static str) -> Result < () , OpenError > { use std :: io :: Read ; let exit_status = child . wait () . map_err (OpenError :: Io) ? ; if exit_status . success () { Ok (()) } else { let mut stderr_output = String :: new () ; if let Some (stderr) = child . stderr . as_mut () { stderr . read_to_string (& mut stderr_output) . ok () ; } Err (OpenError :: ExitStatus { cmd : cmd_name , status : exit_status , stderr : stderr_output , }) } }
    };
}

wait_child!()