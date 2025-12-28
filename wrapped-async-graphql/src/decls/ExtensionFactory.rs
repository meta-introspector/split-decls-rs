macro_rules! deps {
    () => {
        Extension!();
    };
}

macro_rules! ExtensionFactory {
    () => {
        deps!();
        # [doc = " Extension factory"] # [doc = ""] # [doc = " Used to create an extension instance."] pub trait ExtensionFactory : Send + Sync + 'static { # [doc = " Create an extended instance."] fn create (& self) -> Arc < dyn Extension > ; }
    };
}

ExtensionFactory!()