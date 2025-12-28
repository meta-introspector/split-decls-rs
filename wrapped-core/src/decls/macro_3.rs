macro_rules! macro_3 {
    () => {
        # [cfg (windows)] include ! ("windows.rs") ;
    };
}

macro_3!();