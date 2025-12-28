macro_rules! deps {
    () => {
        JoinAllKind!();
        JoinAll!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl < F > fmt :: Debug for JoinAll < F > where F : Future + fmt :: Debug , F :: Output : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . kind { JoinAllKind :: Small { ref elems } => { f . debug_struct ("JoinAll") . field ("elems" , elems) . finish () } # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] JoinAllKind :: Big { ref fut , .. } => fmt :: Debug :: fmt (fut , f) , } } }
    };
}

impl_220!()