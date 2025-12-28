macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! IntoCString {
    () => {
        deps!();
        # [doc = " A class of types that can be converted to C strings."] # [doc = ""] # [doc = " These types are represented internally as byte slices and it is quite rare"] # [doc = " for them to contain an interior 0 byte."] pub trait IntoCString { # [doc = " Consume this container, converting it into a CString"] fn into_c_string (self) -> Result < CString , Error > ; }
    };
}

IntoCString!()