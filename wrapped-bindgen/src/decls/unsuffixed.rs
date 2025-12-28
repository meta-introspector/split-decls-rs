macro_rules! unsuffixed {
    () => {
        macro_rules ! unsuffixed { ($ ty : ty => $ name : ident) => { pub fn $ name (n : $ ty) -> Self { Self { inner : n . to_string () , } } } ; }
    };
}

unsuffixed!()