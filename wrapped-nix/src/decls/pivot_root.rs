macro_rules! deps {
    () => {
        NixPath!();
        Result!();
    };
}

macro_rules! pivot_root {
    () => {
        deps!();
        # [cfg (linux_android)] # [cfg (feature = "fs")] mod pivot_root { use crate :: errno :: Errno ; use crate :: { NixPath , Result } ; # [doc = " Change the root file system."] # [doc = ""] # [doc = " See Also [`pivot_root`](https://man7.org/linux/man-pages/man2/pivot_root.2.html)"] pub fn pivot_root < P1 : ? Sized + NixPath , P2 : ? Sized + NixPath > (new_root : & P1 , put_old : & P2 ,) -> Result < () > { let res = new_root . with_nix_path (| new_root | { put_old . with_nix_path (| put_old | unsafe { libc :: syscall (libc :: SYS_pivot_root , new_root . as_ptr () , put_old . as_ptr () ,) }) }) ? ? ; Errno :: result (res) . map (drop) } }
    };
}

pivot_root!()