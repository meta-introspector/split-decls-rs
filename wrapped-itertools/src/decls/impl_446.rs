macro_rules! deps {
    () => {
        ProcessResults!();
    };
}

macro_rules! impl_446 {
    () => {
        deps!();
        impl < I , E > ProcessResults < '_ , I , E > { # [inline (always)] fn next_body < T > (& mut self , item : Option < Result < T , E > >) -> Option < T > { match item { Some (Ok (x)) => Some (x) , Some (Err (e)) => { * self . error = Err (e) ; None } None => None , } } }
    };
}

impl_446!()