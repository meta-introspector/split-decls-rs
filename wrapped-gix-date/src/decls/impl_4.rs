macro_rules! deps {
    () => {
        CustomFormat!();
        Format!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl From < CustomFormat > for Format { fn from (custom_format : CustomFormat) -> Format { Format :: Custom (custom_format) } }
    };
}

impl_4!()