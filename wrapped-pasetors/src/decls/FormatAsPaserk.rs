macro_rules! FormatAsPaserk {
    () => {
        # [doc = " A trait for serializing a type as PASERK."] pub trait FormatAsPaserk { # [doc = " Format a key as PASERK."] fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result ; }
    };
}

FormatAsPaserk!();