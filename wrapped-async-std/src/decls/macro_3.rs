macro_rules! macro_3 {
    () => {
        cfg_std ! { pub mod io ; pub mod os ; pub mod prelude ; pub mod sync ; pub mod channel ; }
    };
}

macro_3!()