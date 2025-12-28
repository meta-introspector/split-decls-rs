macro_rules! deps {
    () => {
        Compat!();
        Send!();
    };
}

macro_rules! impl_1003 {
    () => {
        deps!();
        impl < Sp , Fut > Executor01 < Fut > for Compat < Sp > where for < 'a > & 'a Sp : Spawn03 , Fut : Future01 < Item = () , Error = () > + Send + 'static , { fn execute (& self , future : Fut) -> Result < () , ExecuteError01 < Fut > > { (& self . inner) . spawn (future . compat () . map (| _ | ())) . expect ("unable to spawn future from Compat executor") ; Ok (()) } }
    };
}

impl_1003!()