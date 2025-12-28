macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! Execv {
    () => {
        deps!();
        pub trait Execv : Send + Sync { fn execv (& self , program : & OsStr , args : & [& OsStr] , current_dir : Option < & Path >) -> Result < Output > ; }
    };
}

Execv!()