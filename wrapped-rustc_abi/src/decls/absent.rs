macro_rules! deps {
    () => {
        LayoutData!();
    };
}

macro_rules! absent {
    () => {
        deps!();
        fn absent < 'a , FieldIdx , VariantIdx , F > (fields : & IndexSlice < FieldIdx , F >) -> bool where FieldIdx : Idx , VariantIdx : Idx , F : Deref < Target = & 'a LayoutData < FieldIdx , VariantIdx > > + fmt :: Debug , { let uninhabited = fields . iter () . any (| f | f . is_uninhabited ()) ; let is_1zst = fields . iter () . all (| f | f . is_1zst ()) ; uninhabited && is_1zst }
    };
}

absent!();