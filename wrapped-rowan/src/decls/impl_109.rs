macro_rules! deps {
    () => {
        TokenAtOffset!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < T > Iterator for TokenAtOffset < T > { type Item = T ; fn next (& mut self) -> Option < T > { match std :: mem :: replace (self , TokenAtOffset :: None) { TokenAtOffset :: None => None , TokenAtOffset :: Single (node) => { * self = TokenAtOffset :: None ; Some (node) } TokenAtOffset :: Between (left , right) => { * self = TokenAtOffset :: Single (right) ; Some (left) } } } fn size_hint (& self) -> (usize , Option < usize >) { match self { TokenAtOffset :: None => (0 , Some (0)) , TokenAtOffset :: Single (_) => (1 , Some (1)) , TokenAtOffset :: Between (_ , _) => (2 , Some (2)) , } } }
    };
}

impl_109!();