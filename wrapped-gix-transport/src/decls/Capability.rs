macro_rules! Capability {
    () => {
        # [doc = " The name of a single capability."] pub struct Capability < 'a > (& 'a BStr) ;
    };
}

Capability!()