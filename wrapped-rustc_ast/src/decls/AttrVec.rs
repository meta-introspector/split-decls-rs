macro_rules! deps {
    () => {
        Attribute!();
    };
}

macro_rules! AttrVec {
    () => {
        deps!();
        # [doc = " A list of attributes."] pub type AttrVec = ThinVec < Attribute > ;
    };
}

AttrVec!()