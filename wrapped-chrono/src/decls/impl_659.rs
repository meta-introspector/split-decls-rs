macro_rules! deps {
    () => {
        Transition!();
    };
}

macro_rules! impl_659 {
    () => {
        deps!();
        # [cfg (windows)] impl PartialOrd for Transition { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_659!()