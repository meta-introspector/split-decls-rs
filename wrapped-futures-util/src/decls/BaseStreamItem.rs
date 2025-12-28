macro_rules! BaseStreamItem {
    () => {
        type BaseStreamItem < St > = < NestedTryStreamIntoEitherTryStream < St > as Stream > :: Item ;
    };
}

BaseStreamItem!();