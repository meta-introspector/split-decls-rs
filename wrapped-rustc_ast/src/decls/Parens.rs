macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! Parens {
    () => {
        deps!();
        # [doc = " Whether enclosing parentheses are present or not."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum Parens { Yes , No , }
    };
}

Parens!()