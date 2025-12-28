macro_rules! deps {
    () => {
        InFileWrapper!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < FileKind , L , R > InFileWrapper < FileKind , Either < L , R > > { pub fn transpose (self) -> Either < InFileWrapper < FileKind , L > , InFileWrapper < FileKind , R > > { match self . value { Either :: Left (l) => Either :: Left (InFileWrapper :: new (self . file_id , l)) , Either :: Right (r) => Either :: Right (InFileWrapper :: new (self . file_id , r)) , } } }
    };
}

impl_87!()