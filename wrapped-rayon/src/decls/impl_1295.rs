macro_rules! deps {
    () => {
        UnindexedProducer!();
        CharsProducer!();
        Folder!();
    };
}

macro_rules! impl_1295 {
    () => {
        deps!();
        impl < 'ch > UnindexedProducer for CharsProducer < 'ch > { type Item = char ; fn split (self) -> (Self , Option < Self >) { match split (self . chars) { Some ((left , right)) => (CharsProducer { chars : left } , Some (CharsProducer { chars : right }) ,) , None => (self , None) , } } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { folder . consume_iter (self . chars . chars ()) } }
    };
}

impl_1295!()