macro_rules! Convert {
    () => {
        # [doc (hidden)] pub trait Convert < T > { fn convert (& self) -> T ; }
    };
}

Convert!();