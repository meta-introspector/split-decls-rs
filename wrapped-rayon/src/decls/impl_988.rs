macro_rules! deps {
    () => {
        WalkTreePrefixProducer!();
        Folder!();
        IntoIter!();
        UnindexedProducer!();
    };
}

macro_rules! impl_988 {
    () => {
        deps!();
        impl < S , B , I > UnindexedProducer for WalkTreePrefixProducer < '_ , S , B > where S : Send , B : Fn (& S) -> I + Send + Sync , I : IntoIterator < Item = S , IntoIter : DoubleEndedIterator > , { type Item = S ; fn split (mut self) -> (Self , Option < Self >) { while self . to_explore . len () == 1 { let front_node = self . to_explore . pop () . unwrap () ; self . to_explore . extend ((self . children_of) (& front_node) . into_iter () . rev ()) ; self . seen . push (front_node) ; } let right_children = split_vec (& mut self . to_explore) ; let right = right_children . map (| mut c | { std :: mem :: swap (& mut c , & mut self . to_explore) ; WalkTreePrefixProducer { to_explore : c , seen : Vec :: new () , children_of : self . children_of , } }) . or_else (| | { let right_seen = split_vec (& mut self . seen) ; right_seen . map (| s | WalkTreePrefixProducer { to_explore : Default :: default () , seen : s , children_of : self . children_of , }) }) ; (self , right) } fn fold_with < F > (mut self , mut folder : F) -> F where F : Folder < Self :: Item > , { folder = folder . consume_iter (self . seen) ; if folder . full () { return folder ; } while let Some (e) = self . to_explore . pop () { self . to_explore . extend ((self . children_of) (& e) . into_iter () . rev ()) ; folder = folder . consume (e) ; if folder . full () { return folder ; } } folder } }
    };
}

impl_988!()