macro_rules! ConstantDeref {
    () => {
        # [doc (hidden)] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct ConstantDeref < T > (T) ;
    };
}

ConstantDeref!()