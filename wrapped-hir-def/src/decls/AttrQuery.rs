macro_rules! deps {
    () => {
        Attrs!();
    };
}

macro_rules! AttrQuery {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct AttrQuery < 'attr > { attrs : & 'attr Attrs , key : Symbol , }
    };
}

AttrQuery!();