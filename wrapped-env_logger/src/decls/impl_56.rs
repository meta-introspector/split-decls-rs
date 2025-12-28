macro_rules! deps {
    () => {
        DefaultVisitSource!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < 'kvs > VisitSource < 'kvs > for DefaultVisitSource < '_ > { fn visit_pair (& mut self , key : Key < '_ > , value : Value < 'kvs >) -> Result < () , Error > { write ! (self . 0 , " {}={}" , self . style_key (key) , value) ? ; Ok (()) } }
    };
}

impl_56!();