macro_rules! deps {
    () => {
        Formatter!();
        WritableTarget!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl std :: fmt :: Debug for WritableTarget { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , match self { Self :: WriteStdout => "stdout" , Self :: PrintStdout => "stdout" , Self :: WriteStderr => "stderr" , Self :: PrintStderr => "stderr" , Self :: Pipe (_) => "pipe" , }) } }
    };
}

impl_30!()