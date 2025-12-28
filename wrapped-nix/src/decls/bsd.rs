macro_rules! bsd {
    () => {
        # [cfg (any (bsd , solarish , target_os = "haiku" ,))] # [macro_use] mod bsd ;
    };
}

bsd!();