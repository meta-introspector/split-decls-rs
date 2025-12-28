macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! RangeSyntax {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum RangeSyntax { # [doc = " `...`"] DotDotDot , # [doc = " `..=`"] DotDotEq , }
    };
}

RangeSyntax!();