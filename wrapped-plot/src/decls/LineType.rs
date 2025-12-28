macro_rules! LineType {
    () => {
        # [doc = " Line type"] # [allow (missing_docs)] # [derive (Clone , Copy)] pub enum LineType { Dash , Dot , DotDash , DotDotDash , # [doc = " Line made of minimally sized dots"] SmallDot , Solid , }
    };
}

LineType!()