macro_rules! deps {
    () => {
        AnnNode!();
        State!();
    };
}

macro_rules! PpAnn {
    () => {
        deps!();
        pub trait PpAnn { fn pre (& self , _state : & mut State < '_ > , _node : AnnNode < '_ >) { } fn post (& self , _state : & mut State < '_ > , _node : AnnNode < '_ >) { } }
    };
}

PpAnn!();