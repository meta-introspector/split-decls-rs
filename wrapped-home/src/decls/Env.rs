macro_rules! Env {
    () => {
        # [doc = " Permits parameterizing the home functions via the _from variants - used for"] # [doc = " in-process unit testing by rustup."] pub trait Env { # [doc = " Return the path to the users home dir, or None if any error occurs:"] # [doc = " see `home_inner`."] fn home_dir (& self) -> Option < PathBuf > ; # [doc = " Return the current working directory."] fn current_dir (& self) -> io :: Result < PathBuf > ; # [doc = " Get an environment variable, as per `std::env::var_os`."] fn var_os (& self , key : & str) -> Option < OsString > ; }
    };
}

Env!()