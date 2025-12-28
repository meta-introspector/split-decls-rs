macro_rules! macro_2 {
    () => {
        cfg_alloc ! { pub mod task ; pub mod future ; pub mod stream ; }
    };
}

macro_2!()