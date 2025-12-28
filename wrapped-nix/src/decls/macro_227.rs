macro_rules! deps {
    () => {
        Result!();
        NixPath!();
    };
}

macro_rules! macro_227 {
    () => {
        deps!();
        feature ! { #! [feature = "acct"] # [doc = " Process accounting"] # [cfg (not (any (target_os = "redox" , target_os = "haiku" , target_os = "cygwin")))] pub mod acct { use crate :: errno :: Errno ; use crate :: { NixPath , Result } ; use std :: ptr ; # [doc = " Enable process accounting"] # [doc = ""] # [doc = " See also [acct(2)](https://linux.die.net/man/2/acct)"] pub fn enable < P : ? Sized + NixPath > (filename : & P) -> Result < () > { let res = filename . with_nix_path (| cstr | unsafe { libc :: acct (cstr . as_ptr ()) }) ?; Errno :: result (res) . map (drop) } # [doc = " Disable process accounting"] pub fn disable () -> Result < () > { let res = unsafe { libc :: acct (ptr :: null ()) } ; Errno :: result (res) . map (drop) } } }
    };
}

macro_227!()