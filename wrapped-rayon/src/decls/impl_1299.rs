macro_rules! deps {
    () => {
        UnindexedProducer!();
        CharIndicesProducer!();
        Folder!();
    };
}

macro_rules! impl_1299 {
    () => {
        deps!();
        impl < 'ch > UnindexedProducer for CharIndicesProducer < 'ch > { type Item = (usize , char) ; fn split (self) -> (Self , Option < Self >) { match split (self . chars) { Some ((left , right)) => (CharIndicesProducer { chars : left , .. self } , Some (CharIndicesProducer { chars : right , index : self . index + left . len () , }) ,) , None => (self , None) , } } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { let base = self . index ; folder . consume_iter (self . chars . char_indices () . map (offset (base))) } }
    };
}

impl_1299!();