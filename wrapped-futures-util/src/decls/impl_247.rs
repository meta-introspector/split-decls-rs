macro_rules! deps {
    () => {
        TryJoinAll!();
        TryJoinAllKind!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl < F > fmt :: Debug for TryJoinAll < F > where F : TryFuture + fmt :: Debug , F :: Ok : fmt :: Debug , F :: Error : fmt :: Debug , F :: Output : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . kind { TryJoinAllKind :: Small { ref elems } => { f . debug_struct ("TryJoinAll") . field ("elems" , elems) . finish () } # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] TryJoinAllKind :: Big { ref fut , .. } => fmt :: Debug :: fmt (fut , f) , } } }
    };
}

impl_247!()