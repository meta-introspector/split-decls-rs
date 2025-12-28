macro_rules! deps {
    () => {
        ExprId!();
    };
}

macro_rules! Array {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub enum Array { ElementList { elements : Box < [ExprId] > } , Repeat { initializer : ExprId , repeat : ExprId } , }
    };
}

Array!()