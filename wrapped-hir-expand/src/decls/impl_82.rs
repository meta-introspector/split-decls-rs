macro_rules! deps {
    () => {
        InFileWrapper!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < FileKind : Copy , T > InFileWrapper < FileKind , T > { pub fn with_value < U > (& self , value : U) -> InFileWrapper < FileKind , U > { InFileWrapper :: new (self . file_id , value) } pub fn as_ref (& self) -> InFileWrapper < FileKind , & T > { self . with_value (& self . value) } pub fn borrow < U > (& self) -> InFileWrapper < FileKind , & U > where T : Borrow < U > , { self . with_value (self . value . borrow ()) } }
    };
}

impl_82!();