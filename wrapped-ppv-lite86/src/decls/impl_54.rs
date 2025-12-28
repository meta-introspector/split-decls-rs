macro_rules! impl_54 {
    () => {
        impl < W : Not + Copy > Not for x4 < W > { type Output = x4 < W :: Output > ; # [inline (always)] fn not (self) -> Self :: Output { x4 ([self . 0 [0] . not () , self . 0 [1] . not () , self . 0 [2] . not () , self . 0 [3] . not () ,]) } }
    };
}

impl_54!()