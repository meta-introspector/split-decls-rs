macro_rules! deps {
    () => {
        Formatter!();
        Target!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Target { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , match self { Self :: Stdout => "stdout" , Self :: Stderr => "stderr" , Self :: Pipe (_) => "pipe" , }) } }
    };
}

impl_33!()