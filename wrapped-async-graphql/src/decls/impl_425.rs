macro_rules! deps {
    () => {
        FieldValueInner!();
        Result!();
        FieldValue!();
    };
}

macro_rules! impl_425 {
    () => {
        deps!();
        impl Debug for FieldValue < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match & self . 0 { FieldValueInner :: Value (v) => write ! (f , "{}" , v) , FieldValueInner :: BorrowedAny (ty , _) | FieldValueInner :: OwnedAny (ty , _) | FieldValueInner :: WithType { ty , .. } => write ! (f , "{}" , ty) , FieldValueInner :: List (list) => match list . first () { Some (v) => { write ! (f , "[{:?}, ...]" , v) } None => { write ! (f , "[()]") } } , } } }
    };
}

impl_425!();