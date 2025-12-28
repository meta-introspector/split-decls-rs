macro_rules! deps {
    () => {
        Attr!();
    };
}

macro_rules! RawAttrs {
    () => {
        deps!();
        # [doc = " Syntactical attributes, without filtering of `cfg_attr`s."] # [derive (Default , Debug , Clone , PartialEq , Eq)] pub struct RawAttrs { entries : Option < ThinArc < () , Attr > > , }
    };
}

RawAttrs!()