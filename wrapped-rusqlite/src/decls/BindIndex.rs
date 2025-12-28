macro_rules! deps {
    () => {
        Result!();
        Error!();
        Statement!();
    };
}

macro_rules! BindIndex {
    () => {
        deps!();
        # [doc = " A trait implemented by types that can index into parameters of a statement."] # [doc = ""] # [doc = " It is only implemented for `usize` and `&str` and `&CStr`."] pub trait BindIndex : sealed :: Sealed { # [doc = " Returns the index of the associated parameter, or `Error` if no such"] # [doc = " parameter exists."] fn idx (& self , stmt : & Statement < '_ >) -> Result < usize > ; }
    };
}

BindIndex!();