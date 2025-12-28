macro_rules! piped {
    () => {
        # [doc = " Creates new pipes for stderr, stdout, and optionally stdin."] fn piped (cmd : & mut Command , pipe_stdin : bool) -> & mut Command { cmd . stdout (Stdio :: piped ()) . stderr (Stdio :: piped ()) . stdin (if pipe_stdin { Stdio :: piped () } else { Stdio :: null () }) }
    };
}

piped!()