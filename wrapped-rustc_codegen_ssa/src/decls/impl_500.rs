macro_rules! deps {
    () => {
        OperandRef!();
    };
}

macro_rules! impl_500 {
    () => {
        deps!();
        impl < V : CodegenObject > fmt :: Debug for OperandRef < '_ , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "OperandRef({:?} @ {:?})" , self . val , self . layout) } }
    };
}

impl_500!()