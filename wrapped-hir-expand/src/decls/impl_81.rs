macro_rules! deps {
    () => {
        InFileWrapper!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < FileKind , T > InFileWrapper < FileKind , T > { pub fn new (file_id : FileKind , value : T) -> Self { Self { file_id , value } } pub fn map < F : FnOnce (T) -> U , U > (self , f : F) -> InFileWrapper < FileKind , U > { InFileWrapper :: new (self . file_id , f (self . value)) } }
    };
}

impl_81!();