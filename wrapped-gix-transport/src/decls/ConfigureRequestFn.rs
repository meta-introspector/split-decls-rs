macro_rules! deps {
    () => {
        Request!();
        Error!();
    };
}

macro_rules! ConfigureRequestFn {
    () => {
        deps!();
        # [doc = " A function to configure a single request prior to sending it, support most complex configuration beyond what's possible with"] # [doc = " basic `git` http configuration."] pub type ConfigureRequestFn = dyn FnMut (& mut reqwest :: blocking :: Request) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > + Send + Sync + 'static ;
    };
}

ConfigureRequestFn!();