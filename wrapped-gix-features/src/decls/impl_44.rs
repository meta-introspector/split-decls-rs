macro_rules! deps {
    () => {
        SequenceId!();
        InOrderIter!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < T , E , I > From < I > for InOrderIter < T , I > where I : Iterator < Item = Result < (SequenceId , T) , E > > , { fn from (iter : I) -> Self { InOrderIter { inner : iter , store : Default :: default () , next_chunk : 0 , is_done : false , } } }
    };
}

impl_44!();