macro_rules! deps {
    () => {
        Sink!();
        FnMut1!();
    };
}

macro_rules! impl_386 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < St , F , Item > Sink < Item > for Map < St , F > where St : Stream + Sink < Item > , F : FnMut1 < St :: Item > , { type Error = St :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_386!();