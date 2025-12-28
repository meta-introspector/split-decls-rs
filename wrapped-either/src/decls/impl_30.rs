macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < L : Clone , R : Clone > Clone for Either < L , R > { fn clone (& self) -> Self { match self { Left (inner) => Left (inner . clone ()) , Right (inner) => Right (inner . clone ()) , } } fn clone_from (& mut self , source : & Self) { match (self , source) { (Left (dest) , Left (source)) => dest . clone_from (source) , (Right (dest) , Right (source)) => dest . clone_from (source) , (dest , source) => * dest = source . clone () , } } }
    };
}

impl_30!()