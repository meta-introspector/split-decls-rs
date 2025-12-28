macro_rules! deps {
    () => {
        File!();
        Marker!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl File { # [doc = " Obtain a mutable reference to the write handle and call `f(out)` with it."] pub fn with_mut < T > (& mut self , f : impl FnOnce (& mut std :: fs :: File) -> std :: io :: Result < T >) -> std :: io :: Result < T > { self . inner . with_mut (| tf | f (tf . as_file_mut ())) . and_then (| res | res) } # [doc = " Close the lock file to prevent further writes and to save system resources."] # [doc = " A call to [`Marker::commit()`] is allowed on the [`Marker`] to write changes back to the resource."] pub fn close (self) -> std :: io :: Result < Marker > { Ok (Marker { inner : self . inner . close () ? , created_from_file : true , lock_path : self . lock_path , }) } # [doc = " Return the path at which the lock file resides"] pub fn lock_path (& self) -> & Path { & self . lock_path } # [doc = " Return the path at which the locked resource resides"] pub fn resource_path (& self) -> PathBuf { strip_lock_suffix (& self . lock_path) } }
    };
}

impl_20!();