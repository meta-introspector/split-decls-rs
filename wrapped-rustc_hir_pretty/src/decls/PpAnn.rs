macro_rules! deps {
    () => {
        Nested!();
        AnnNode!();
        State!();
    };
}

macro_rules! PpAnn {
    () => {
        deps!();
        pub trait PpAnn { fn nested (& self , _state : & mut State < '_ > , _nested : Nested) { } fn pre (& self , _state : & mut State < '_ > , _node : AnnNode < '_ >) { } fn post (& self , _state : & mut State < '_ > , _node : AnnNode < '_ >) { } }
    };
}

PpAnn!()