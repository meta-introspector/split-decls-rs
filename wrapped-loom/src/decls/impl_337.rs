macro_rules! deps {
    () => {
        JoinHandle!();
        Thread!();
    };
}

macro_rules! impl_337 {
    () => {
        deps!();
        impl < T > JoinHandle < T > { # [doc = " Waits for the associated thread to finish."] # [track_caller] pub fn join (self) -> std :: thread :: Result < T > { self . notify . wait (location ! ()) ; self . result . lock () . unwrap () . take () . unwrap () } # [doc = " Gets a handle to the underlying [`Thread`]"] pub fn thread (& self) -> & Thread { & self . thread } }
    };
}

impl_337!();