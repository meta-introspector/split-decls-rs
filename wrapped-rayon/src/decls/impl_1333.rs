macro_rules! deps {
    () => {
        MatchesProducer!();
        Folder!();
        UnindexedProducer!();
    };
}

macro_rules! impl_1333 {
    () => {
        deps!();
        impl < 'ch , 'pat , P : Pattern > UnindexedProducer for MatchesProducer < 'ch , 'pat , P > { type Item = & 'ch str ; fn split (self) -> (Self , Option < Self >) { match split (self . chars) { Some ((left , right)) => (MatchesProducer { chars : left , .. self } , Some (MatchesProducer { chars : right , .. self }) ,) , None => (self , None) , } } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { self . pattern . fold_matches (self . chars , folder) } }
    };
}

impl_1333!()