macro_rules! deps {
    () => {
        Executor!();
    };
}

macro_rules! unblock {
    () => {
        deps!();
        # [doc = " Runs blocking code on a thread pool."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Read the contents of a file:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use blocking::unblock;"] # [doc = " use std::fs;"] # [doc = ""] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let contents = unblock(|| fs::read_to_string(\"file.txt\")).await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] # [doc = ""] # [doc = " Spawn a process:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use blocking::unblock;"] # [doc = " use std::process::Command;"] # [doc = ""] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let out = unblock(|| Command::new(\"dir\").output()).await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub fn unblock < T , F > (f : F) -> Task < T > where F : FnOnce () -> T + Send + 'static , T : Send + 'static , { Executor :: spawn (async move { f () }) }
    };
}

unblock!();