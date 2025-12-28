macro_rules! OperandValueBuilder {
    () => {
        # [doc = " Each of these variants starts out as `Either::Right` when it's uninitialized,"] # [doc = " then setting the field changes that to `Either::Left` with the backend value."] # [derive (Debug , Copy , Clone)] enum OperandValueBuilder < V > { ZeroSized , Immediate (Either < V , abi :: Scalar >) , Pair (Either < V , abi :: Scalar > , Either < V , abi :: Scalar >) , # [doc = " `repr(simd)` types need special handling because they each have a non-empty"] # [doc = " array field (which uses [`OperandValue::Ref`]) despite the SIMD type itself"] # [doc = " using [`OperandValue::Immediate`] which for any other kind of type would"] # [doc = " mean that its one non-ZST field would also be [`OperandValue::Immediate`]."] Vector (Either < V , () >) , }
    };
}

OperandValueBuilder!()