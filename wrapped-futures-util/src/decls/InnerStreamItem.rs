macro_rules! deps {
    () => {
        BaseStreamItem!();
    };
}

macro_rules! InnerStreamItem {
    () => {
        deps!();
        type InnerStreamItem < St > = < BaseStreamItem < St > as Stream > :: Item ;
    };
}

InnerStreamItem!()