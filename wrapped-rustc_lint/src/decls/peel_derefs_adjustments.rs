macro_rules! peel_derefs_adjustments {
    () => {
        # [doc = " Peel derefs adjustments until the last last element."] fn peel_derefs_adjustments < 'a > (mut adjs : & 'a [Adjustment < 'a >]) -> & 'a [Adjustment < 'a >] { while let [Adjustment { kind : Adjust :: Deref (_) , .. } , end @ ..] = adjs && ! end . is_empty () { adjs = end ; } adjs }
    };
}

peel_derefs_adjustments!()