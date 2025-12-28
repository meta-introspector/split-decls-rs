macro_rules! deps {
    () => {
        Separator!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl PartialEq for Separator { fn eq (& self , other : & Separator) -> bool { use Separator :: * ; match (self , other) { (Ident (a) , Ident (b)) => a . sym == b . sym , (Literal (a) , Literal (b)) => a . symbol == b . symbol , (Puncts (a) , Puncts (b)) if a . len () == b . len () => { let a_iter = a . iter () . map (| a | a . char) ; let b_iter = b . iter () . map (| b | b . char) ; a_iter . eq (b_iter) } (Lifetime (_ , a) , Lifetime (_ , b)) => a . sym == b . sym , _ => false , } } }
    };
}

impl_18!()