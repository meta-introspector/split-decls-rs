macro_rules! deps {
    () => {
        InFileWrapper!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < FileKind , T > InFileWrapper < FileKind , Option < T > > { pub fn transpose (self) -> Option < InFileWrapper < FileKind , T > > { Some (InFileWrapper :: new (self . file_id , self . value ?)) } }
    };
}

impl_86!();