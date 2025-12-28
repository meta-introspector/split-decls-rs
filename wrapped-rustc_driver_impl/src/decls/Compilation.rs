macro_rules! Compilation {
    () => {
        # [doc = " Whether to stop or continue compilation."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum Compilation { Stop , Continue , }
    };
}

Compilation!();