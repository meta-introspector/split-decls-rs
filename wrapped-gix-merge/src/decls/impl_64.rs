macro_rules! deps {
    () => {
        Data!();
        Resource!();
        ResourceRef!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < 'a > ResourceRef < 'a > { pub (super) fn new (cache : & 'a Resource) -> Self { ResourceRef { data : cache . data . map_or (Data :: Missing , | data | match data { pipeline :: Data :: Buffer => Data :: Buffer (& cache . buffer) , pipeline :: Data :: TooLarge { size } => Data :: TooLarge { size } , }) , rela_path : cache . rela_path . as_ref () , id : & cache . id , } } }
    };
}

impl_64!();