macro_rules! deps {
    () => {
        RepeatProducer!();
        UnindexedProducer!();
        Folder!();
    };
}

macro_rules! impl_800 {
    () => {
        deps!();
        impl < T : Clone + Send > UnindexedProducer for RepeatProducer < T > { type Item = T ; fn split (self) -> (Self , Option < Self >) { (RepeatProducer { element : self . element . clone () , } , Some (RepeatProducer { element : self . element , }) ,) } fn fold_with < F > (self , folder : F) -> F where F : Folder < T > , { folder . consume_iter (iter :: repeat (self . element)) } }
    };
}

impl_800!();