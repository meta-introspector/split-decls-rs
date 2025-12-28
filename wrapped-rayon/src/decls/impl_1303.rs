macro_rules! deps {
    () => {
        UnindexedProducer!();
        Folder!();
        BytesProducer!();
    };
}

macro_rules! impl_1303 {
    () => {
        deps!();
        impl < 'ch > UnindexedProducer for BytesProducer < 'ch > { type Item = u8 ; fn split (self) -> (Self , Option < Self >) { match split (self . chars) { Some ((left , right)) => (BytesProducer { chars : left } , Some (BytesProducer { chars : right }) ,) , None => (self , None) , } } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { folder . consume_iter (self . chars . bytes ()) } }
    };
}

impl_1303!()