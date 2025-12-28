macro_rules! macro_27 {
    () => {
        cfg_unstable ! { pub mod pin ; # [cfg (all (not (target_os = "unknown") , feature = "std"))] pub mod process ; mod unit ; mod vec ; mod result ; mod option ; mod string ; mod collections ; }
    };
}

macro_27!()