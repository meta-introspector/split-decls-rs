macro_rules! deps {
    () => {
        InFileWrapper!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < FileKind : Copy , T : Clone > InFileWrapper < FileKind , & T > { pub fn cloned (& self) -> InFileWrapper < FileKind , T > { self . with_value (self . value . clone ()) } }
    };
}

impl_83!()