macro_rules! Fallibility {
    () => {
        # [doc = " Whether memory allocation errors should return an error or abort."] # [derive (Copy , Clone)] enum Fallibility { Fallible , Infallible , }
    };
}

Fallibility!();