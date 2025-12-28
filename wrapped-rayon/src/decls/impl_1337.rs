macro_rules! deps {
    () => {
        Folder!();
        MatchIndicesProducer!();
        UnindexedProducer!();
    };
}

macro_rules! impl_1337 {
    () => {
        deps!();
        impl < 'ch , 'pat , P : Pattern > UnindexedProducer for MatchIndicesProducer < 'ch , 'pat , P > { type Item = (usize , & 'ch str) ; fn split (self) -> (Self , Option < Self >) { match split (self . chars) { Some ((left , right)) => (MatchIndicesProducer { chars : left , .. self } , Some (MatchIndicesProducer { chars : right , index : self . index + left . len () , .. self }) ,) , None => (self , None) , } } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { self . pattern . fold_match_indices (self . chars , folder , self . index) } }
    };
}

impl_1337!();