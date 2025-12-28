macro_rules! deps {
    () => {
        EnvCompleter!();
    };
}

macro_rules! Shells {
    () => {
        deps!();
        # [doc = " Collection of shell-specific completers"] pub struct Shells < 's > (pub & 's [& 's dyn EnvCompleter]) ;
    };
}

Shells!();