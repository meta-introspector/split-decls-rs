macro_rules! deps {
    () => {
        OnUpgrade!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl OnUpgrade { pub (super) fn none () -> Self { OnUpgrade { rx : None } } # [cfg (all (any (feature = "client" , feature = "server") , feature = "http1"))] pub (super) fn is_none (& self) -> bool { self . rx . is_none () } }
    };
}

impl_218!()