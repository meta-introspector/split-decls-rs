macro_rules! impl_282 {
    () => {
        impl < St1 , St2 > Chain < St1 , St2 > where St1 : Stream , St2 : Stream < Item = St1 :: Item > , { pub (super) fn new (stream1 : St1 , stream2 : St2) -> Self { Self { first : Some (stream1) , second : stream2 } } }
    };
}

impl_282!()