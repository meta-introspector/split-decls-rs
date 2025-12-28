macro_rules! deps {
    () => {
        NodeOrToken!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < N , T > NodeOrToken < N , T > { pub fn into_node (self) -> Option < N > { match self { NodeOrToken :: Node (node) => Some (node) , NodeOrToken :: Token (_) => None , } } pub fn into_token (self) -> Option < T > { match self { NodeOrToken :: Node (_) => None , NodeOrToken :: Token (token) => Some (token) , } } pub fn as_node (& self) -> Option < & N > { match self { NodeOrToken :: Node (node) => Some (node) , NodeOrToken :: Token (_) => None , } } pub fn as_token (& self) -> Option < & T > { match self { NodeOrToken :: Node (_) => None , NodeOrToken :: Token (token) => Some (token) , } } }
    };
}

impl_101!()